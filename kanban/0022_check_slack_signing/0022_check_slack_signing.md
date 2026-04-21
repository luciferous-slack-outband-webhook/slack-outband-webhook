# workerにSlackの署名検証を実装する
## 要望

Slack AppのSubscribeの際に署名の検証がセキュリティ上推奨される。

これを実装して欲しい。

Signing SecretはCloudflare WorkerのSecret `SLACK_APP_SIGNING_SECRET`として登録した。

## プラン

1. `worker/Cargo.toml` に `hmac = "0.12"`, `sha2 = "0.10"`, `hex = "0.4"` を追加
2. `worker/src/lib.rs` に署名検証関数 `verify_slack_signature` を実装
   - ヘッダー取得 → タイムスタンプ検証 → ボディ読み取り → HMAC-SHA256 計算 → 定数時間比較
   - 失敗時は 401 を返す
3. `handle_post` で `verify_slack_signature` を呼び出す

## 完了サマリー

- 完了日時: 2026-04-13T21:59:46+09:00
- `worker/Cargo.toml` に `hmac`, `sha2`, `hex` を追加
- `worker/src/lib.rs` に `verify_slack_signature` を実装し、`handle_post` に組み込んだ
- HMAC-SHA256 による署名計算、タイムスタンプ検証（5分以内）、定数時間比較を実装
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` および `make cargo-check` でビルド確認済み