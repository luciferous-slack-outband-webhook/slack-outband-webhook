# ログ: 0010 Cloudflare WorkerのデプロイでAccountIDを指定するようにする

## 開始日時
2026-04-12T23:46:36+09:00

## 実装内容

### 変更ファイル
- `.github/workflows/deploy.yml`

### 変更内容
`wrangler-action@v3` の `with:` に `accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}` を追加。
GitHub Secrets 経由で渡すことで、アカウントIDをハードコードせずにデプロイ先を明示できる。

### 備考
GitHub リポジトリの Settings > Secrets に `CLOUDFLARE_ACCOUNT_ID` を設定する必要がある。

## 完了日時
2026-04-12T23:46:45+09:00
