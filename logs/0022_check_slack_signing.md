# 0022 workerにSlackの署名検証を実装する

## ヘッダー

- 開始時刻: 2026-04-13T21:58:38+09:00
- 完了時刻: 2026-04-13T21:59:46+09:00

## タスク概要

Slack AppのSubscribeの際に署名の検証がセキュリティ上推奨される。これを実装する。
Signing SecretはCloudflare WorkerのSecret `SLACK_APP_SIGNING_SECRET`として登録済み。

## 調査結果

- `worker/src/lib.rs`: スタブ状態。POST `/` ルートが固定JSON `{"msg":"Hello, World!"}` を返すのみ
- `worker/Cargo.toml`: `worker = "0.8"` と `serde` のみ。暗号ライブラリは未追加
- Cloudflare Workers (wasm32-unknown-unknown) で動作するため、pure Rust の暗号ライブラリが必要
- `worker::Date::now().as_millis()` で現在時刻取得可能
- Secret は `ctx.secret("SLACK_APP_SIGNING_SECRET")?.to_string()` で取得可能
- `req.text().await` はボディの一度読みなので、ヘッダー取得後に呼ぶ必要がある

## 実装プラン

1. `worker/Cargo.toml` に `hmac = "0.12"`, `sha2 = "0.10"`, `hex = "0.4"` を追加
2. `worker/src/lib.rs` に署名検証関数 `verify_slack_signature` を実装
   - ヘッダー取得 → タイムスタンプ検証 → ボディ読み取り → HMAC-SHA256 計算 → 定数時間比較
   - 失敗時は 401 を返す
3. `handle_post` で `verify_slack_signature` を呼び出す

## 実装フェーズ

### 編集したファイル

- `worker/Cargo.toml`: `hmac = "0.12"`, `sha2 = "0.10"`, `hex = "0.4"` を追加
- `worker/src/lib.rs`: 署名検証関数 `verify_slack_signature` を実装、`handle_post` を更新

### 実行したコマンド

- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` → 成功
- `make cargo-check` → 全チェック通過

### 判断・意思決定

- 暗号ライブラリは `hmac 0.12` + `sha2 0.10` を採用（pure Rust、wasm32-unknown-unknown で動作、`digest 0.10` を共有する安定した組み合わせ）
- `verify_slack_signature` の戻り値型を `std::result::Result<String, Result<Response>>` とし、ボディを呼び出し元に返す設計（`req.text()` は一度しか呼べないため）
- 署名の比較は `mac.verify_slice()` で定数時間比較（タイミング攻撃防止）

### エラー・問題

- 特になし
