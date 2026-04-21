# ログ: 0009 デプロイ時もRustのCacheを有効にする

- 開始: 2026-04-12T15:34:09+09:00
- 完了: 2026-04-12T15:34:20+09:00

## ステップ

### 1. deploy.yml に Swatinem/rust-cache@v2 を追加

- `.github/workflows/deploy.yml` の `dtolnay/rust-toolchain` ステップと `cloudflare/wrangler-action` ステップの間に追加
- `cache-directories: "~/.cargo/bin"` を指定して `worker-build` バイナリもキャッシュ対象に含めた
