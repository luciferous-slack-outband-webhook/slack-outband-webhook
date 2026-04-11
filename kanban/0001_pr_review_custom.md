# Claude CodeによるPRレビューをカスタマイズする
## 要望
以前別リポジトリでClaude CodeによるPRレビューを行っていたが、色々カスタマイズした。

それをもとにこのリポジトリもカスタマイズしてほしい。

追加の要望は以下の通り
- PRのコメントにおいて日本語で行うようにもカスタマイズしてほしい
- PRコメントの最上部に、Claudeの使用量がどの程度残っているかも表示して欲しい

## プラン

1. `.github/workflows/claude-code-review.yml` の `plugin_marketplaces` / `plugins` を廃止し、カスタムプロンプトに差し替え
2. `permissions.pull-requests: read` → `write` に昇格（コメント投稿・編集のため）
3. プロンプトに日本語レビュー指示・過去コメント確認手順を追加
4. `Extract usage from execution file` ステップで `execution_file` から jq で使用量情報を抽出
5. `Prepend usage block to Claude's PR comment` ステップで Bot コメントの先頭に使用量ブロックを prepend

注記: 「月間残量」は OAuth トークン経由の公式 API が無いため、当該ジョブの消費量（tokens/cost）表示に変更。

## 以前使ったGithub Actionsのワークフロー定義
```yaml
name: Claude Code Review

on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
    # Optional: Only run on specific file changes
    # paths:
    #   - "src/**/*.ts"
    #   - "src/**/*.tsx"
    #   - "src/**/*.js"
    #   - "src/**/*.jsx"

jobs:
  claude-review:
    # Optional: Filter by PR author
    # if: |
    #   github.event.pull_request.user.login == 'external-contributor' ||
    #   github.event.pull_request.user.login == 'new-developer' ||
    #   github.event.pull_request.author_association == 'FIRST_TIME_CONTRIBUTOR'

    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write
      issues: read
      id-token: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 1

      - name: Run Claude Code Review
        id: claude-review
        uses: anthropics/claude-code-action@v1
        with:
          claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}
          prompt: |
            このPRをレビューしてください。

            ## 前提: 過去のレビューコメントの確認
            レビューを開始する前に、まず以下のコマンドで過去のレビューコメントとその返信を確認してください：

            1. PRのレビューコメント一覧を取得:
               gh api repos/${{ github.repository }}/pulls/${{ github.event.pull_request.number }}/comments

            2. PRの一般コメント一覧を取得:
               gh api repos/${{ github.repository }}/issues/${{ github.event.pull_request.number }}/comments

            3. PRのレビュー一覧を取得:
               gh api repos/${{ github.repository }}/pulls/${{ github.event.pull_request.number }}/reviews

            過去のレビューで指摘された問題が修正されているか確認し、同じ指摘を繰り返さないようにしてください。
            まだ対応されていない過去の指摘があれば、その旨をコメントに含めてください。

            ## レビュー観点
            以下の観点でチェックし、レビューコメントを投稿してください：
            - コード品質とベストプラクティス
            - 潜在的なバグや問題
            - セキュリティへの影響
            - パフォーマンス上の考慮事項
          claude_args: "--allowedTools Bash(gh api *) Read Glob Grep"


```
## 完了サマリー

- **完了日時**: 2026-04-11T23:51:54+09:00
- **変更ファイル**: `.github/workflows/claude-code-review.yml`

### 実施内容

1. **プラグイン廃止 → カスタムプロンプト**: `plugin_marketplaces` / `plugins` を削除し、以前のリポジトリで使っていたプロンプト構成をベースに差し替え
2. **日本語化**: プロンプトに「レビュー本文は必ず日本語で出力してください」を明示追加
3. **過去レビュー確認**: PR のコメント・レビュー一覧を `gh api` で取得し、同じ指摘を繰り返さないよう指示
4. **権限昇格**: `pull-requests: write` に変更（コメント投稿・編集のため）
5. **使用量表示**: `Extract usage from execution file` ステップで `execution_file` の `type=="result"` 行から使用量を抽出し、`Prepend usage block to Claude's PR comment` ステップで Bot コメント先頭に引用ブロック形式で prepend

### 注記

- 「月間残量」の取得は OAuth トークン経由の公式 API が存在しないため、当該ジョブ消費量（入力/出力トークン・キャッシュ・コスト・実行時間）に変更
- 両後続ステップは `continue-on-error: true` でワークフロー全体を保護
- E2E 確認（実際に PR を立ててワークフロー動作確認）はユーザー側で実施が必要
