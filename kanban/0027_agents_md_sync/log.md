# 0027_agents_md_sync 作業ログ

## 基本情報

- **タスクファイル**: kanban/0027_agents_md_sync.md
- **開始日時**: 2026-04-19T16:56:13+09:00
- **完了日時**: 2026-04-19T16:57:19+09:00

## タスク概要

CLAUDE.md を更新したのに AGENTS.md に追従させる仕組みが動かなかった。修正して欲しい。

## 調査結果

### `.claude/settings.json`

PostToolUse hook が設定されている:
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "bash .claude/hooks/sync-agents-md.sh"
          }
        ]
      }
    ]
  }
}
```
Edit または Write ツール使用後に `sync-agents-md.sh` が実行される設計になっている。

### `.claude/hooks/sync-agents-md.sh`

問題の核心: スクリプト冒頭で `SHARED_SECTIONS=("Overview" "Commands" "Architecture")` とホワイトリストをハードコードしていた。

```bash
SHARED_SECTIONS=("Overview" "Commands" "Architecture")
```

このリストに含まれるセクションだけが CLAUDE.md から AGENTS.md に同期される。CLAUDE.md に後から追加されたセクションは自動では同期されない。

### CLAUDE.md の現状（5セクション）

1. Overview
2. Commands
3. Architecture
4. CI  ← SHARED_SECTIONS に含まれていない
5. Kanban ワークフロー  ← SHARED_SECTIONS に含まれていない

### AGENTS.md の現状（4セクション）

1. Overview（同期済み）
2. Commands（同期済み）
3. Architecture（同期済み）
4. Review guidelines（AGENTS.md 固有）

→ 「CI」「Kanban ワークフロー」が AGENTS.md に存在しない

### コミット e482e62 での変更

`e482e62` で CLAUDE.md の「Kanban ワークフロー」セクションに以下が追記された:
```
- `kanban/` と `logs/` は開発者向けの記録であり、Codex Code Review の対象外としている（`.github/workflows/codex-code-review.yml` で作業ツリーから削除している）
```
この変更は hook が実行されたが、SHARED_SECTIONS に「Kanban ワークフロー」が無いためスキップされた。

### スクリプト導入時期

`eab4fbf`（2026-04-13）で sync-agents-md.sh と settings.json が同時に追加された。
CLAUDE.md に「CI」セクションはそれ以前から存在していた可能性が高く（履歴確認）、追加時点ですでに同期対象外だった。

## 実装プラン

### 採用アプローチ: ホワイトリスト → ブラックリスト方式への変更

`SHARED_SECTIONS`（同期するセクション名のホワイトリスト）を廃止し、
`EXCLUDE_SECTIONS`（同期しないセクション名のブラックリスト）に置き換える。

- CLAUDE.md の全 `##` セクションを出現順で AGENTS.md に同期する
- EXCLUDE_SECTIONS に含まれるセクションはスキップ（現時点では空）
- AGENTS.md 固有セクション（CLAUDE.md に存在しないもの）は末尾に保持
- 先頭の `This file provides guidance to OpenAI Codex...` は固定出力

### 検討した代替案

A. SHARED_SECTIONS に「CI」「Kanban ワークフロー」を手動で追加
  → 今後 CLAUDE.md に新セクションを追加するたびに手動更新が必要。根本解決にならない。却下。

B. ブラックリスト方式（採用）
  → 新セクションが自動的に同期されるため、「自動追従」という本来の目的を達成できる。

### extract_section 関数

既存の関数をそのまま流用する。

## プランニング経緯

### 初回提案

- ホワイトリスト廃止、ブラックリスト方式への変更
- CLAUDE.md 全セクションを同期（除外リストに含まれる場合のみスキップ）
- AGENTS.md 固有セクションを末尾に保持

### ユーザーフィードバック

初回提案がそのまま承認された。

## 会話内容

### [16:56] ユーザー指示

`/kanban` コマンド実行（引数なし）。未完了タスクのうち最新番号（0027）を自動選択。

### [16:56] Claude 対応

kanban/0027_agents_md_sync.md を読み込み、調査開始。
- settings.json で hook 設定を確認
- sync-agents-md.sh を確認し SHARED_SECTIONS ホワイトリスト問題を特定
- CLAUDE.md / AGENTS.md のセクション差異を確認
- git log で e482e62 の変更内容を確認
- プランモードに入り、ブラックリスト方式への変更計画を作成・承認取得

## 編集したファイル

| ファイル | 変更内容 |
|---------|---------|
| `.claude/hooks/sync-agents-md.sh` | ホワイトリスト → ブラックリスト方式に変更 |
| `AGENTS.md` | スクリプト手動実行で再生成 |

## 実行したコマンド

```bash
# スクリプト手動実行（AGENTS.md 再生成）
CWD=/Users/yuta/space/projects/luciferous-slack-outband-webhook/00_repos/slack-outband-webhook
echo "{\"tool_input\":{\"file_path\":\"$CWD/CLAUDE.md\"},\"cwd\":\"$CWD\"}" | bash "$CWD/.claude/hooks/sync-agents-md.sh"
# → [sync-agents-md] AGENTS.md を CLAUDE.md の変更に合わせて更新しました
```

## 判断・意思決定

- `EXCLUDE_SECTIONS=()` は現時点では空で運用。CLAUDE.md のどのセクションも Codex に見せて害がない一般情報のため。

## エラー・問題

-（なし）

## 会話内容（フェーズ2）

### [16:56] 実装開始

ログファイル作成後、`.claude/hooks/sync-agents-md.sh` を書き換え。
SHARED_SECTIONS ホワイトリストを廃止し、EXCLUDE_SECTIONS ブラックリスト（空）方式に変更。

### [16:57] スクリプト手動実行

修正済みスクリプトを stdin JSON で起動。正常終了を確認。

### [16:57] git diff 確認

`git diff AGENTS.md` で以下を確認:
- 「CI」「Kanban ワークフロー」セクションが CLAUDE.md と同じ内容で追加された
- 「Review guidelines」セクションはそのまま保持された
- 冒頭の `This file provides guidance to OpenAI Codex...` は維持された
