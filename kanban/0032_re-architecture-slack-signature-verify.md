# Slackのsignature verifyを別ファイルに分離しテストを追加する
## 要望
開発のやり方など、別枠でレビューを受けていたのですが、Slackのsignature verifyについてテストを入れることを提案された。

そのために、ファイルを分割し、テスト可能にするという案を受けたので対応して欲しい。

## 目的
[SWE-CI](https://arxiv.org/abs/2603.03823) という論文を知って、今の開発のやり方はどうかというレビューをしてもらった。
これは長期的に保守する上での能力についての論文で、対応する価値があると判断した。

その上で、受けた指摘について対応して欲しい。

## 以下指摘

なるほど、`shared` の位置づけ了解です。であれば僕の「sharedのテスト」は的外れで、焦点は純粋に **worker の署名検証**一本です。あと `worker/Cargo.toml` も確認したので、それを踏まえて具体的に書きます。

## なぜここから始めるべきか

改めて整理すると、`verify_slack_signature` は次の特徴を持っています。

- **セキュリティ critical** — ここが壊れると Slack 以外からのリクエストを受け付ける、もしくはリプレイ攻撃を通す
- **仕様が外部で固定** — Slack公式の署名仕様は勝手に変わらない。テストを書くROIが長期に渡って維持される
- **正解が決定論的** — HMAC-SHA256なので、入力が同じなら出力は完全に一致する。flaky testにならない
- **人間のレビューで見落とされやすい** — 暗号処理は一見正しく見えても微妙に間違える典型箇所(例: 定数時間比較を `==` でやってしまう)

SWE-CI的に言えば、ここは「将来 Claude/Codex が『こっちの書き方の方が簡潔です』と提案して、気付かぬうちに脆弱化する」可能性が最も高い場所です。テストでピン留めしておく価値が極めて高い。

## テストを書く上での構造的な壁

`worker/Cargo.toml` を見ると次のようになっています。

```toml
[lib]
crate-type = ["cdylib"]
```

これが問題で、2つの壁があります。

**壁1**: `crate-type = ["cdylib"]` だけだと `cargo test` が素直に通らない。native ターゲットで test binary を作るには `rlib` も必要です。

**壁2**: `worker` crate 自体が wasm32-unknown-unknown 向けで、`Request` / `Response` / `RouteContext` といった型はネイティブでは使えない。つまり現在の `verify_slack_signature(req: &mut Request, ctx: &RouteContext<()>)` はそのままではテスト不能。

## 処方箋: ロジックを純粋関数に切り出す

解決策は **I/O 境界と暗号ロジックを分離する** ことです。具体的には:

```
worker/src/
├── lib.rs      # worker エントリポイント + I/O 境界
└── slack.rs    # 純粋ロジック(worker crate に依存しない)
```

`slack.rs` は `worker` crate を import せず、プリミティブ型だけ受け取る設計にします。

```rust
// worker/src/slack.rs
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const TIMESTAMP_TOLERANCE_SECONDS: u64 = 5 * 60;

#[derive(Debug, PartialEq)]
pub enum VerifyError {
    TimestampParseError,
    TimestampTooOld,
    InvalidSignatureFormat,
    InvalidHex,
    SignatureMismatch,
    HmacInitError,
}

pub fn verify_slack_signature(
    timestamp: &str,
    signature: &str,
    body: &str,
    signing_secret: &[u8],
    now_seconds: u64,
) -> Result<(), VerifyError> {
    let ts: u64 = timestamp.parse().map_err(|_| VerifyError::TimestampParseError)?;

    if now_seconds.abs_diff(ts) > TIMESTAMP_TOLERANCE_SECONDS {
        return Err(VerifyError::TimestampTooOld);
    }

    let expected_signature = signature
        .strip_prefix("v0=")
        .ok_or(VerifyError::InvalidSignatureFormat)?;
    let expected_bytes =
        hex::decode(expected_signature).map_err(|_| VerifyError::InvalidHex)?;

    let sig_basestring = format!("v0:{timestamp}:{body}");
    let mut mac =
        HmacSha256::new_from_slice(signing_secret).map_err(|_| VerifyError::HmacInitError)?;
    mac.update(sig_basestring.as_bytes());
    mac.verify_slice(&expected_bytes)
        .map_err(|_| VerifyError::SignatureMismatch)?;

    Ok(())
}
```

ポイントは次の通りです:

- **時刻を引数で注入** (`now_seconds: u64`) することで、`Date::now()` を排除してテスト可能に
- **エラーを列挙型で分類** することで、「なぜ弾いたか」をテストで主張できる(現在のコードは`worker::Response`を返すのでテストから見ると不透明)
- **`worker` crate への依存ゼロ** なのでネイティブで普通にコンパイル・テスト可能

`lib.rs` 側は I/O 境界だけ残します:

```rust
// worker/src/lib.rs
mod slack;
use slack::{verify_slack_signature, VerifyError};
use worker::*;

async fn handle_post(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let timestamp = req.headers().get("X-Slack-Request-Timestamp").ok().flatten()
        .ok_or_else(|| worker::Error::from("Missing X-Slack-Request-Timestamp"))?;
    let signature = req.headers().get("X-Slack-Signature").ok().flatten()
        .ok_or_else(|| worker::Error::from("Missing X-Slack-Signature"))?;
    let body = req.text().await?;
    let secret = ctx.secret("SLACK_APP_SIGNING_SECRET")?.to_string();
    let now = Date::now().as_millis() / 1000;

    match verify_slack_signature(&timestamp, &signature, &body, secret.as_bytes(), now) {
        Ok(()) => { /* handle webhook */ }
        Err(VerifyError::TimestampTooOld) => return Response::error("Timestamp too old", 401),
        Err(_) => return Response::error("Unauthorized", 401),
    }
    // ...
}
```

エラー種別を本番レスポンスにどこまで出すかは議論の余地ありですが(細かく出すと偵察されやすい)、少なくとも内部のエラー識別はテスト可能になります。

## Cargo.toml の変更

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dev-dependencies]
# 追加は不要 - hmac/sha2/hex は通常依存で既にある
```

`rlib` を足すだけで `cargo test -p slack-outband-webhook-worker --target x86_64-...` が動くようになります。

## テストすべきケース一覧

純粋関数にすると、以下が普通の `#[cfg(test)]` で書けます。

| ケース | 期待する結果 | 理由 |
|---|---|---|
| 正しい署名・タイムスタンプ | `Ok(())` | 正常系 |
| 署名の1バイト変更 | `SignatureMismatch` | 改ざん検出 |
| `timestamp = now - 301` (6分前) | `TimestampTooOld` | リプレイ境界 |
| `timestamp = now - 299` (5分前) | `Ok(())` | 境界内は通る |
| `timestamp = now + 301` (未来) | `TimestampTooOld` | **abs_diff なので未来方向も弾く** — 意図通りか要確認 |
| `signature = "xxx"` (v0= 無し) | `InvalidSignatureFormat` | |
| `signature = "v0=notahex"` | `InvalidHex` | |
| `timestamp = "abc"` | `TimestampParseError` | |
| `body` が空文字列 | 署名一致ならOK | 空bodyは正常なイベントではないが panic しないこと |
| `signing_secret` が空 | HMACは技術的には計算可能 | `HmacInitError` は `new_from_slice` が失敗する時のみ — 実質到達しない |

**署名サンプルの作り方**: 既知の secret と timestamp+body でHMAC-SHA256を手計算した hex をテストに埋め込む。Slack公式ドキュメントの example value を使うか、同じロジックの別言語実装(例: Slack公式の Node.js SDK)で正解を生成してテストfixtureに入れる。

## テストコードのイメージ

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"8f742231b10e8888abcd99yyyzzz85a5";
    const TS: &str = "1531420618";
    const BODY: &str = "token=xyzz0WbapA4vBCDEFasx0q6G&team_id=T1DC2JH3J&...";

    fn compute_sig(secret: &[u8], ts: &str, body: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret).unwrap();
        mac.update(format!("v0:{ts}:{body}").as_bytes());
        format!("v0={}", hex::encode(mac.finalize().into_bytes()))
    }

    #[test]
    fn accepts_valid_signature() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now: u64 = TS.parse().unwrap();
        assert_eq!(verify_slack_signature(TS, &sig, BODY, SECRET, now), Ok(()));
    }

    #[test]
    fn rejects_tampered_body() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now: u64 = TS.parse().unwrap();
        assert_eq!(
            verify_slack_signature(TS, &sig, "tampered", SECRET, now),
            Err(VerifyError::SignatureMismatch)
        );
    }

    #[test]
    fn rejects_old_timestamp() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now: u64 = TS.parse::<u64>().unwrap() + 301;
        assert_eq!(
            verify_slack_signature(TS, &sig, BODY, SECRET, now),
            Err(VerifyError::TimestampTooOld)
        );
    }

    #[test]
    fn rejects_future_timestamp() {
        let sig = compute_sig(SECRET, TS, BODY);
        let now: u64 = TS.parse::<u64>().unwrap() - 301;
        assert_eq!(
            verify_slack_signature(TS, &sig, BODY, SECRET, now),
            Err(VerifyError::TimestampTooOld)
        );
    }
    // ... 他のケース
}
```

## Stop hook との組み合わせ

前回話した Stop hook のテストゲートは、Makefile の `cargo-check` ターゲットを拡張するのが綺麗です。

```makefile
cargo-check:
	cargo check -p shared -p cli
	cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown

cargo-test:
	cargo test -p shared -p cli
	cargo test -p slack-outband-webhook-worker  # native target (rlib 経由)

verify: cargo-check cargo-test
	cargo clippy --all-targets -- -D warnings

.PHONY: cargo-check cargo-test verify
```

そして `.claude/settings.json` にこれを追加:

```json
"Stop": [{
  "hooks": [{
    "type": "agent",
    "prompt": "make verify を実行し、全てパスすることを確認せよ。失敗したら 'ok: false, reason: <失敗内容>' を返し、成功したら 'ok: true' を返せ。",
    "timeout": 300
  }]
}]
```

これで「Claudeがコード変更してセッションを終えようとするたびに、workerのテストが強制実行される」状態になります。SWE-CIで測っている zero-regression rate を個人プロジェクトレベルで強制する仕組みです。

## 完了サマリー

**完了日時**: 2026-04-20T14:10:56+09:00

- `worker/Cargo.toml`: `crate-type = ["cdylib", "rlib"]` に変更し、native ターゲットのテスト実行を可能にした
- `worker/src/slack.rs` を新規作成: `VerifyError` enum + `verify_slack_signature` pure function + 10テストケース全件
- `worker/src/lib.rs` を改修: I/O 境界のみ残し `slack::verify_slack_signature` に委譲。HTTP レスポンスは現状維持。
- `Makefile`: `cargo-test` ターゲット追加
- `cargo test -p slack-outband-webhook-worker` → 10テスト全パス
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` → 成功
- `cargo clippy -p slack-outband-webhook-worker --target wasm32-unknown-unknown -- -D warnings` → 警告なし
- `cargo fmt --all --check` → フォーマット OK

## プラン

1. `worker/Cargo.toml`: `crate-type = ["cdylib", "rlib"]` に変更
2. `worker/src/slack.rs` 新規作成: `VerifyError` enum + `verify_slack_signature` pure function + 10テストケース
3. `worker/src/lib.rs`: `mod slack;` 宣言、I/O 境界のみ残し `slack::verify_slack_signature` に委譲。エラーレスポンスは現状維持。
4. `Makefile`: `cargo-test` ターゲット追加（CI ワークフローは別タスク）

テスト: kanban 提案表の全 10 ケース（正常系/改ざん/タイムスタンプ境界/未来/フォーマット/hex/parse/空body/空secret）。

## 実装の順序の提案

実装始めたばかりとのことなので、一気にやるより段階的に:

1. **今**: `slack.rs` への切り出しだけやる(機能追加ゼロ、純粋なリファクタ)
2. **次**: 正常系+改ざん検出の2テストだけ書く(最低限のピン留め)
3. **実装が進んだら**: エッジケースを埋める + Stop hook を導入
4. **KV連携実装時**: `shared` の型に対する serde round-trip test を書く(ここが Yutaさんが言われていた shared の役割ですね)

段階1だけでも、**この先Claudeが`verify_slack_signature`を「簡潔にリファクタしました」と言って構造を壊す**のを防げます。というのも純粋関数にした時点で、lib.rs 側の I/O 境界と slack.rs 側のロジックの契約が明示的になるので、勝手な変更が目立ちます。

何かここで補足/反論ありますか? 特に「純粋関数への切り出しは Rust/wasm の設計として正しいか」という点は Yutaさんの方が経験あると思うので、疑問点あれば突っ込んでください。