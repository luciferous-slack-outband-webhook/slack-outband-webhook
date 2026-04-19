# 0030 Date::now().as_millis() ビルド不能レビュー対応

- 開始: 2026-04-19T18:12:43+09:00
- 完了: 2026-04-19T18:13:18+09:00

---

## タスク概要

Codex レビューから以下の P1 指摘が来た:

> **P1 🟠**: Date::now().as_millis() が存在せずビルド不能
>
> `worker::Date::now()` はミリ秒の `f64` を返す API であり `as_millis()` メソッドは存在しないためコンパイルが通りません。結果として署名検証が利用できず、ワーカーがビルド不能になります。既存のレビュー指摘が未解消です。
>
> 信頼度: 0.74

現状 `cargo check` が通っているため、ビルド不能というのはないと思われるが、指摘が来たため調査・対応する。

---

## 調査結果

### worker/Cargo.toml

```toml
[dependencies]
worker = "0.8"
```

### Cargo.lock 確認

```
name = "worker"
version = "0.8.0"
```

→ `worker` crate の実際のバージョンは `0.8.0` に固定されている。

### worker/src/lib.rs:63

```rust
let now_seconds = Date::now().as_millis() / 1000;
```

`use worker::*;` により `worker::Date` が使われている。

### worker 0.8.0 のソース確認

`/Users/yuta/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/worker-0.8.0/src/date.rs` を直接確認:

```rust
// date.rs:17
pub struct Date {
    js_date: JsDate,
}

// date.rs:44-68 (impl Date)
impl Date {
    pub fn new(init: DateInit) -> Self {
        let val = match init {
            DateInit::Millis(n) => JsValue::from_f64(n as f64),
            DateInit::String(s) => JsValue::from_str(&s),
        };
        Self { js_date: JsDate::new(&val) }
    }

    /// Get the current time, represented by a Date.
    pub fn now() -> Self {
        Self { js_date: JsDate::new_0() }
    }

    /// Convert a Date into its number of milliseconds since the Unix epoch.
    pub fn as_millis(&self) -> u64 {
        self.js_date.get_time() as u64
    }
}
```

**`Date::now()` は `Self`（`Date` 構造体）を返す。`f64` ではない。**
**`as_millis(&self) -> u64` メソッドは worker 0.8.0 に存在する。**

→ Codex レビューコメント（信頼度 0.74）は**誤り**。

### CI ワークフロー (.github/workflows/build.yml)

```yaml
- name: Check native crates
  run: cargo check -p shared -p cli

- name: Check worker crate (wasm32)
  run: cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown
```

CI の "Check worker crate (wasm32)" ステップが `wasm32-unknown-unknown` ターゲットで `cargo check` を実行しており、これが通っている。

---

## 実装プラン

コード変更なし。`worker::Date::now().as_millis()` は worker 0.8.0 に存在する正しい API 呼び出し。

検証として `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` を実行し、ビルドが通ることを確認する。

---

## プランニング経緯

初回提案がそのまま承認された。

プラン: コード変更不要、ビルド検証のみ実施。

---

## 会話内容

1. ユーザーが「worker::Date::now().as_millis()が存在せずビルド不能というレビューコメントが来た」と要望
2. kanban/0030_fix_date_as_millis.md を作成
3. プランモードに入り調査
4. `worker/src/lib.rs:63` で `Date::now().as_millis()` を確認
5. `worker` crate 0.8.0 のソース（`date.rs:58, 65`）で `now() -> Self` と `as_millis() -> u64` の存在を確認
6. CI ワークフローが `wasm32-unknown-unknown` で `cargo check` していることを確認
7. コード変更不要のプランを提案、承認

---

## 実装フェーズ

### 実行したコマンド

```
cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown
```

結果: 成功

```
Checking slack-outband-webhook-worker v0.0.2
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
```

### 編集したファイル

なし

### 判断・意思決定

- `worker::Date::now().as_millis()` は worker 0.8.0 に存在するため、コード変更不要と判断
- Codex レビュー（信頼度 0.74）は誤りと確定

### エラー・問題

なし
