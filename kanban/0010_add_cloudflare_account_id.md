# Cloudflare WorkerのデプロイでAccountIDを指定するようにする
## 要望
```
**1. `deploy.yml`: `account_id` が未設定でデプロイ先が曖昧**

`wrangler-action` の `apiToken` のみが指定されており、`wrangler.toml` にも `account_id` が記載されていません。Cloudflare API トークンが複数アカウントに紐づいている場合、デプロイ先アカウントが不定になります。`wrangler.toml` に `account_id` を追加するか、`deploy.yml` の `with:` に `accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}` を追加してください。

```

PRのレビューで上記指摘があった。
account idを指定するようにして欲しい。

## プラン
- `deploy.yml` の `wrangler-action@v3` の `with:` に `accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}` を追加
- アカウントIDは GitHub Secrets 経由で渡す（ハードコード避け）

## 完了サマリー
2026-04-12T23:46:45+09:00 完了

`.github/workflows/deploy.yml` の `wrangler-action@v3` に `accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}` を追加した。
GitHub リポジトリの Settings > Secrets に `CLOUDFLARE_ACCOUNT_ID` を設定すれば、デプロイ先アカウントが明示されるようになる。