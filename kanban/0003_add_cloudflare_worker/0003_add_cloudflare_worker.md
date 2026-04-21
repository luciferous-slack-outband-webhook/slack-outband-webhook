# cloudflare workerを作る
## 要望
`worker/` をCloudflare Worker用に実装してください。

ルートパスでPOSTを受け取り、JSONで `{"msg": "Hello, World!"}` と返してください。

また、デプロイ用のGithub Actionsのワークフローも書いてください。
デプロイ用のワークフローは `v*` へのタグプッシュで起動するようにしてください。

workerの名前は `slack-outband-webhook` としてください。

## プラン

- `worker/Cargo.toml` のパッケージ名を `slack-outband-webhook-worker` に変更し、`[lib] crate-type = ["cdylib"]` と `worker`/`serde` 依存を追加
- `worker/src/main.rs` を削除し `worker/src/lib.rs` を新規作成（`#[event(fetch)]` + Router で POST `/` → JSON レスポンス）
- `worker/wrangler.toml` を新規作成（Worker 名・build コマンド設定）
- `.gitignore` に `worker/build/` 追加・Cargo.lock の無視解除
- `.github/workflows/deploy.yml` を新規作成（`v*` タグプッシュで Cloudflare Workers デプロイ）

---

## 完了サマリー

- **完了日時**: 2026-04-12T00:57:34+09:00
- **対応内容**:
  - `worker/` を Cloudflare Worker（cdylib）として実装
  - POST `/` で `{"msg": "Hello, World!"}` を返す `#[event(fetch)]` ハンドラを作成
  - `worker/wrangler.toml` で Worker 名 `slack-outband-webhook` を設定
  - `v*` タグプッシュでデプロイする `.github/workflows/deploy.yml` を作成
  - `.gitignore` に `worker/build/` 追加・Cargo.lock を追跡対象に変更
- **変更ファイル**:
  - `worker/Cargo.toml`
  - `worker/src/lib.rs`（新規）
  - `worker/wrangler.toml`（新規）
  - `.gitignore`
  - `.github/workflows/deploy.yml`（新規）
- **備考**: `worker/src/main.rs` を削除（cdylib への変換のため）