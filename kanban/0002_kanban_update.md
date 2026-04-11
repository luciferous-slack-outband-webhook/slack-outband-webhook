# kanbanコマンドのアップデート
## 要望

kanbanコマンドで作成、修正したファイルを自動でコミットするようにしたい。

要望や編集内容から、コミットのタイトルや説明を自動生成し、コミットまでしてほしい。

## プラン

- `/kanban` フェーズ2末尾にステップ5「自動コミット」を追加する
- ステージ対象はタスクで触ったファイルのみ（kanban ファイル・ログ・ソース）に限定し `git add -A` は使用しない
- コミットメッセージは Claude が `## 完了サマリー` と「要望」を基にターン内で生成する
- サンドボックス制約のため `dangerouslyDisableSandbox: true` で git コマンドを実行する
- 変更するファイル: `.claude/commands/kanban.md`、`.claude/kanban-workflow.md`、`CLAUDE.md`

---

## 完了サマリー

- **完了日時**: 2026-04-12T00:06:49+09:00
- **対応内容**:
  - `.claude/commands/kanban.md` のフェーズ2に「ステップ5: 自動コミット」を追加（ステージ対象の選別・コミットメッセージ生成・サンドボックス対応・確認手順）
  - `.claude/kanban-workflow.md` に「自動コミット手順」セクションを新設（ステージ選別ルール・メッセージテンプレート・サンドボックス注意書き）
  - `CLAUDE.md` の Kanban ワークフロー箇条書きに自動コミットの説明を追記
- **変更ファイル**:
  - `.claude/commands/kanban.md`
  - `.claude/kanban-workflow.md`
  - `CLAUDE.md`
  - `kanban/0002_kanban_update.md`
  - `logs/0002_kanban_update.md`