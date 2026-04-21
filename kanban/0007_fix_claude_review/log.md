# Log: 0007_fix_claude_review

## 開始日時
2026-04-12T14:03:03+09:00

## 概要
`claude-code-review.yml` の OIDC 検証エラーを `pull_request_target` への移行で修正する。

## 作業ログ

### Step 1: ログファイル作成
2026-04-12T14:03:03+09:00

### Step 2: ワークフローファイル修正
2026-04-12T14:03:19+09:00

編集ファイル: `.github/workflows/claude-code-review.yml`

変更内容:
1. `pull_request` → `pull_request_target` (line 4)
2. job に `if: github.event.pull_request.head.repo.full_name == github.repository` を追加 (line 9)
3. checkout に `ref: ${{ github.event.pull_request.head.sha }}` を追加 (line 21)

## 完了日時
2026-04-12T14:03:19+09:00
