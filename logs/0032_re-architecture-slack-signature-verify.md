# 0032 Slack署名検証ロジックの純粋関数化とテスト追加

## ヘッダー

- **開始時刻**: 2026-04-20T14:07:28+09:00
- **完了時刻**: 2026-04-20T14:10:56+09:00

## タスク概要

kanban 要望より転記:

> 開発のやり方など、別枠でレビューを受けていたのですが、Slackのsignature verifyについてテストを入れることを提案された。
> そのために、ファイルを分割し、テスト可能にするという案を受けたので対応して欲しい。

## 調査結果

### `worker/Cargo.toml`
- `[lib] crate-type = ["cdylib"]` のみ。`rlib` がないため native ターゲットでテストバイナリを生成できない
- `[dev-dependencies]` セクションなし
- 通常依存: `worker = "0.8"`, `serde = { version = "1", features = ["derive"] }`, `hmac = "0.12"`, `sha2 = "0.10"`, `hex = "0.4"`

### `worker/src/lib.rs`
- 1ファイルのみ（`worker/src/` 配下は `lib.rs` だけ）
- `verify_slack_signature(req: &mut Request, ctx: &RouteContext<()>)` が現在の署名検証関数
- `worker::Request` / `RouteContext` に直接依存しており、wasm32 専用型をそのまま受け取る
- `Date::now().as_millis() / 1000` で現在時刻取得（テスト不能な副作用）
- タイムスタンプリプレイチェック → body読み取り → HMAC計算 → 定数時間比較 の順序
- `mac.verify_slice` で定数時間比較（セキュリティ上正しい実装）

### テスト構造（既存）
- `tests/` ディレクトリは存在しない
- `shared/src/lib.rs` に `#[cfg(test)] mod tests { use super::*; #[test] fn it_works() { ... } }` パターンのみ存在

### Makefile
```makefile
SHELL = /usr/bin/env bash -xeuo pipefail

cargo-check:
    cargo check -p shared -p cli
    cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown

.PHONY: \
    cargo-check
```
`cargo-test` ターゲットは未定義。

### CI ワークフロー
- `build.yml`: `cargo check` のみ実行（`cargo test` なし）
- `lint.yml`: `cargo fmt`, `cargo clippy -p shared -p cli`, `cargo clippy -p slack-outband-webhook-worker --target wasm32-unknown-unknown`
- `codex-code-review.yml`: `cargo check` 結果をプロンプトに注入

### Cargo.lock 実バージョン
- `hex` 0.4.3, `hmac` 0.12.1, `sha2` 0.10.9
- `subtle` 2.6.1（hmac の定数時間比較に使用される推移的依存）

## 実装プラン

### 変更ファイル
1. `worker/Cargo.toml` — `crate-type = ["cdylib", "rlib"]` に変更
2. `worker/src/slack.rs`（新規）— `VerifyError` enum + `verify_slack_signature` pure function + 10テストケース
3. `worker/src/lib.rs` — I/O 境界のみ残し `slack::verify_slack_signature` に委譲
4. `Makefile` — `cargo-test` ターゲット追加

### `worker/src/slack.rs` 設計
```rust
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
) -> Result<(), VerifyError>
```
`worker` crate を import しないため native ビルド可能。時刻を引数で注入。

### `worker/src/lib.rs` の VerifyError マッピング
- `TimestampParseError` → "Invalid timestamp" 401
- `TimestampTooOld` → "Timestamp too old" 401
- `InvalidSignatureFormat` / `InvalidHex` → "Invalid signature format" 401
- `SignatureMismatch` → "Invalid signature" 401
- `HmacInitError` → "Server configuration error" 500

既存の HTTP レスポンスと完全に同一の出力を維持する。

### テストケース 10 件
| # | 入力 | 期待 |
|---|---|---|
| 1 | 正しい署名・タイムスタンプ・`now = ts` | `Ok(())` |
| 2 | 正しい署名 + body だけ改ざん | `Err(SignatureMismatch)` |
| 3 | `now = ts + 301`（6分後） | `Err(TimestampTooOld)` |
| 4 | `now = ts + 299`（境界内） | `Ok(())` |
| 5 | `now = ts - 301`（未来方向・abs_diff） | `Err(TimestampTooOld)` |
| 6 | `signature = "xxx"` (v0= なし) | `Err(InvalidSignatureFormat)` |
| 7 | `signature = "v0=notahex"` | `Err(InvalidHex)` |
| 8 | `timestamp = "abc"` | `Err(TimestampParseError)` |
| 9 | `body = ""` で署名一致 | `Ok(())` |
| 10 | `signing_secret = b""` で署名一致 | `Ok(())` |

SECRET/TS/BODY は Slack 公式ドキュメントの example value を使用。

### Makefile
```makefile
cargo-test:
    cargo test -p slack-outband-webhook-worker

.PHONY: \
    cargo-check \
    cargo-test
```

## プランニング経緯

- 初回提案がそのまま承認された。
- ユーザーへの確認事項:
  - テスト範囲: 表の全ケース（10件）
  - エラー粒度: 現状維持に近い形
  - CI 組込み: Makefile には追加するが CI ワークフローは別タスク（ユーザーノート: 「Makefileには組み込む。CIにおいては別のワークフローにしたい。」）

## 会話内容

- ユーザー: `/kanban` スキルを起動
- Claude: kanban/0032（番号最大の未完了タスク）を選択。Explore エージェント2件を並列起動し、コードベース・CI構成・既存テストパターンを調査。
- Claude: AskUserQuestion でテスト範囲・エラー粒度・CI 組込み方針の3点を質問
- ユーザー回答:
  - テスト: 表の全ケース（推奨）
  - エラー粒度: 現状維持に近い形（推奨）
  - CI 組込み: Makefile には追加、CI ワークフローは別タスク
- Claude: ExitPlanMode でプラン承認申請 → ユーザー承認

## 編集したファイル

- `worker/Cargo.toml`: `crate-type = ["cdylib", "rlib"]` に変更
- `worker/src/slack.rs`: 新規作成
- `worker/src/lib.rs`: mod 宣言 + slack::verify_slack_signature 呼び出しに変更
- `Makefile`: cargo-test ターゲット追加

## 実行したコマンド

- `cargo test -p slack-outband-webhook-worker`
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown`
- `cargo clippy -p slack-outband-webhook-worker --target wasm32-unknown-unknown -- -D warnings`
- `cargo fmt --all --check`

## 判断・意思決定

- `worker/src/lib.rs` の I/O 部分で `?` 演算子を使うと `ok_or_else(|| Response::error(...))` の返す型 `Result<Response, worker::Error>` が `worker::Error` に変換できずビルドエラーになった。元の `verify_slack_signature` は `std::result::Result<String, Result<Response>>` 型の別関数として `?` を機能させていた構造だった。解決策として `let-else` パターンと `match` 式で明示的に `return` する方針に変更した。
- `cargo fmt` の差分が多く出た（セミコロン省略・行折り返し）。`cargo fmt` を実行して修正。

## エラー・問題

- rust-analyzer が `slack.rs` を「unlinked-file」と警告したが、`lib.rs` に `mod slack;` を追加済みだったため問題なし（rust-analyzer のキャッシュ遅延）。
- `?` 演算子の型変換エラー（E0277）4件：`ok_or_else(|| Response::error(...))` の結果に `?` を適用すると `Result<Response>` が `worker::Error` に変換できない。`let-else` と `match` で対処した。
