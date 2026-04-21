# CLADEU.mdを更新
## 要望
GithubのPRレビューをClaudeからCodexに切り替える。
CodexはCLAUDE.mdではなく、AGENTS.mdを参照する。

そのため、CLAUDE.mdに変更があったときにAGETNS.mdも適切な形で更新するようにして欲しい。

## プラン

Claude Code の `PostToolUse` フックを使い、CLAUDE.md が Edit/Write された後に `.claude/hooks/sync-agents-md.sh` を自動実行する。同期スクリプトは共通セクション（Overview, Commands, Architecture）を CLAUDE.md から AGENTS.md に反映し、AGENTS.md 固有のセクション（Review guidelines）は保持する。

## 完了サマリー

**完了日時:** 2026-04-13T13:17:31+09:00

### 実施内容

1. **`.claude/hooks/sync-agents-md.sh` を新規作成**
   - PostToolUse フックとして使用
   - `tool_input.file_path` が CLAUDE.md のときのみ実行
   - 共通セクション（Overview, Commands, Architecture）を CLAUDE.md から抽出して AGENTS.md に反映
   - AGENTS.md 固有セクション（Review guidelines 等）は保持

2. **`.claude/settings.json` に PostToolUse フックを追加**
   - matcher: `Edit|Write`
   - command: `bash .claude/hooks/sync-agents-md.sh`