# Claude CodeによるGithub PRレビューでエラーがでる
## 要望
```
Auto-detected mode: agent for event: pull_request
Requesting OIDC token...
Attempt 1 of 3...
OIDC token successfully obtained
Exchanging OIDC token for app token...
Attempt 1 of 3...
App token exchange failed: 401 Unauthorized - Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Attempt 1 failed: Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Retrying in 5 seconds...
Attempt 2 of 3...
App token exchange failed: 401 Unauthorized - Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Attempt 2 failed: Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Retrying in 10 seconds...
Attempt 3 of 3...
App token exchange failed: 401 Unauthorized - Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Attempt 3 failed: Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Operation failed after 3 attempts
Error: Action failed with error: Workflow validation failed. The workflow file must exist and have identical content to the version on the repository's default branch. If you're seeing this on a PR when you first add a code review workflow file to your repository, this is normal and you should ignore this error.
Internal error: directory mismatch for directory "/home/runner/work/_actions/anthropics/claude-code-action/v1/tsconfig.json", fd 4. You don't need to do anything, but this indicates a bug.
Error: Process completed with exit code 1.
Run BUN_BIN="${GITHUB_ACTION_PATH}/bin/bun"
Internal error: directory mismatch for directory "/home/runner/work/_actions/anthropics/claude-code-action/v1/tsconfig.json", fd 4. You don't need to do anything, but this indicates a bug.
No buffered inline comments
```

`.github/workflws/claude-code-review.md` の `Run Claude Code Review`で上記のエラーが出た。
それによりレビューコメントもない。
どうしたらいい？

## プラン
`pull_request_target` イベントに変更することで修正する。

- **原因**: `claude_code_oauth_token` は OIDC 検証を行い、実行中のワークフローファイルが master 上のものと一致しているか確認する。`pull_request` イベントでは PR ブランチからワークフローが実行されるため検証に失敗する。
- **修正**: `pull_request_target` に変更すると、常に master からワークフローが実行され OIDC 検証が通る。
- **セキュリティ**: fork PR には secrets が漏れないよう `if: github.event.pull_request.head.repo.full_name == github.repository` を追加。checkout は PR の HEAD SHA を明示指定。

## 完了サマリー

完了日時: 2026-04-12T14:03:19+09:00

`.github/workflows/claude-code-review.yml` を以下の通り修正した:

1. `on.pull_request` → `on.pull_request_target` に変更
2. job レベルに fork PR ガード条件を追加: `if: github.event.pull_request.head.repo.full_name == github.repository`
3. checkout に `ref: ${{ github.event.pull_request.head.sha }}` を追加（PR コードを正しく取得するため）

**注意**: この変更は master にマージされて初めて有効になる。master にマージ後、PR を作成 or synchronize して動作確認を行うこと。
