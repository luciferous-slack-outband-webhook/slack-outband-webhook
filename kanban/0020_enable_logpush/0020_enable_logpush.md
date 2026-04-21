# Workerでlogpushを有効化する
## 要望
Cloudflare WorkerでLogpushを有効化してください

## プラン

`worker/wrangler.toml` にトップレベル設定 `logpush = true` を追加する。

## 完了サマリー

- 完了日時: 2026-04-13T20:10:05+09:00
- `worker/wrangler.toml` に `logpush = true` を追加した
- Cloudflare Workers の Logpush が有効化され、ログを Logpush ジョブ経由で外部サービスに転送できるようになった