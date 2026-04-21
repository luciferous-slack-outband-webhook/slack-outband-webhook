# ログ: Claude Code PR レビューのカスタマイズ

- **開始**: 2026-04-11T23:50:08+09:00
- **完了**: 2026-04-11T23:51:54+09:00

## ステップ

### 1. ログファイル作成
- 2026-04-11T23:50:08+09:00: ログスケルトン作成

### 2. コードベース調査 / プランニング
- kanban/0001_pr_review_custom.md 読み込み
- 既存ワークフロー (.github/workflows/claude-code-review.yml, claude.yml) 確認
- claude-code-action の action.yml outputs を WebFetch で確認: `execution_file`, `branch_name`, `github_token`, `structured_output`, `session_id` が存在
- CLI リファレンスで `--output-format stream-json` の result メッセージ構造を確認
- ccusage は CI ランナーではローカル履歴が無いため不適と判断
- ユーザーに確認: 「当該ジョブの消費量表示」「claude-code-action に任せる方式」を選択

### 3. ワークフロー更新 (.github/workflows/claude-code-review.yml)
- 2026-04-11T23:50:30+09:00: ファイル全体を書き換え
  - `plugin_marketplaces` / `plugins` を削除し、カスタムプロンプトに変更
  - `permissions.pull-requests: read` → `write` に昇格
  - 日本語レビュー・過去コメント確認手順をプロンプトに追加
  - `claude_args: "--allowedTools Bash(gh api *) Read Glob Grep"` 維持
  - `Extract usage from execution file` ステップ追加 (jq で result 行から usage 情報を抽出)
  - `Prepend usage block to Claude's PR comment` ステップ追加 (gh api で Bot コメントを特定・先頭に使用量ブロックを prepend)

### 4. YAML 構文確認
- 2026-04-11T23:51:10+09:00: python3 で構造チェック → OK (142行、必須キー全て存在)

### 5. jq クエリ検証
- 2026-04-11T23:51:20+09:00: サンプル JSONL でテスト → 使用量ブロックが期待通りのフォーマットで出力
- sed カンマ区切りが macOS で動作しないことを確認 → jq 完結型に変更
- printf で NEW_BODY 組み立てに修正 (インデント問題の解消)

## 変更ファイル
- `.github/workflows/claude-code-review.yml`: プラグイン廃止 → カスタムプロンプト・日本語化・使用量表示

## 注意点
- `execution_file` の実際のスキーマは実ランナー上で確認が必要。形式が異なる場合は jq クエリを調整。
- Bot コメント特定ロジック: `user.type == "Bot"` で絞り込みの最新コメントを選択。将来の action 変更で壊れる可能性があるが `continue-on-error: true` で workflow 全体は保護。
