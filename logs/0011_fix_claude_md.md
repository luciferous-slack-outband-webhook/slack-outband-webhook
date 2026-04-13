# 0011_fix_claude_md ログ

## 開始: 2026-04-13T13:15:21+09:00

## ステップ

### Step 1: sync-agents-md.sh 作成

- `.claude/hooks/sync-agents-md.sh` を新規作成
- PostToolUse フックとして使用
- stdin の JSON から `tool_input.file_path` を取得し、CLAUDE.md の変更のみに反応
- 共通セクション（Overview, Commands, Architecture）を CLAUDE.md から抽出し AGENTS.md に反映
- AGENTS.md 固有セクション（Review guidelines）は保持

### Step 2: settings.json フック追加

- `.claude/settings.json` に `PostToolUse` フックを追加
- matcher: `Edit|Write` で Edit・Write ツール使用後に実行

### Step 3: 動作確認

- CLAUDE.md 以外のファイル → スキップ（出力なし）: OK
- file_path なし → スキップ: OK
- CLAUDE.md の Overview 変更 → AGENTS.md に反映: OK
- AGENTS.md の Review guidelines 保持: OK

## 完了: 2026-04-13T13:17:31+09:00
