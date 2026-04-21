# `/kanban` を コマンドからスキルに変更する
## 要望
今コマンドにしている `/kanban` をこのプロジェクト用のスキルにしてほしい。
それに伴い、スキルとして最適化して欲しい。

## 目的
コマンドでは iOS版のClaudeからRemote Controlで繋ぐときに使用できない。
スキルにすることで実行可能にしたい。

## プラン

- `.claude/skills/kanban/SKILL.md` を新規作成（YAML frontmatter + スキル定義本文）
- `.claude/kanban-workflow.md` を `.claude/skills/kanban/references/kanban-workflow.md` に git mv し、コマンド→スキルの文言置換
- `.claude/commands/kanban.md` を削除
- `CLAUDE.md` を編集（kanban-workflow.md 参照パス更新 + コマンド→スキル）
- `AGENTS.md` は sync-agents-md.sh フックにより CLAUDE.md 編集時に自動同期

---

## 完了サマリー

- **完了日時**: 2026-04-19T17:18:35+09:00
- **対応内容**:
  - `.claude/skills/kanban/SKILL.md` を新規作成（スキル形式に変換、`$ARGUMENTS` → args パラメータ説明、description 最適化）
  - `.claude/kanban-workflow.md` を `.claude/skills/kanban/references/kanban-workflow.md` に git mv（バンドルリソース化）
  - `.claude/commands/kanban.md` を削除
  - `CLAUDE.md` の kanban-workflow.md 参照パスを新パスに更新、「コマンド」→「スキル」に置換
  - `AGENTS.md` は sync-agents-md.sh フックにより自動同期
- **変更ファイル**:
  - `.claude/skills/kanban/SKILL.md`（新規）
  - `.claude/skills/kanban/references/kanban-workflow.md`（.claude/kanban-workflow.md から移動）
  - `.claude/commands/kanban.md`（削除）
  - `CLAUDE.md`
  - `AGENTS.md`（自動同期）