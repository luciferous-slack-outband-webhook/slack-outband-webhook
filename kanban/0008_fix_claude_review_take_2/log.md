# ログ: 0008 まだClaude Code Reviewでエラーが出る

## 開始日時
2026-04-12T14:15:00+09:00

## 調査内容

### エラー内容
`claude-code-review.yml` の Claude Code Review ステップで以下のエラーが発生:
```
OIDC token successfully obtained
Exchanging OIDC token for app token...
App token exchange failed: 401 Unauthorized - Invalid OIDC token
```

前回タスク(0007)の「Workflow validation failed」とは異なるエラー。

### 調査結果
- `id-token: write` パーミッション → 設定済み、問題なし
- `pull_request_target` への変更 → master マージ済み、問題なし
- `CLAUDE_CODE_OAUTH_TOKEN` → 30分前に再設定済み、問題なし（`/install-github-app` で設定）

### 原因の推定
Anthropic 側で Organization/リポジトリの OIDC 認可がされていない可能性が高い。
GitHub OIDC トークンの claims（`repo:luciferous-slack-outband-webhook/slack-outband-webhook`）が
Anthropic のトークン交換エンドポイントで拒否されている。

### 次のアクション
1. **Claude GitHub App** (https://github.com/apps/claude) を Organization `luciferous-slack-outband-webhook` にインストールする
2. インストール後に PR を re-run して動作確認
3. それでもダメなら `anthropic_api_key` 方式への切り替えを検討

## 記録日時
2026-04-12T14:21:17+09:00
