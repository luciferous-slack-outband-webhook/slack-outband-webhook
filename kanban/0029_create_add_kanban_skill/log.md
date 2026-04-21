# 0029_create_add_kanban_skill 作業ログ

## 基本情報

- **タスクファイル**: kanban/0029_create_add_kanban_skill.md
- **開始日時**: 2026-04-19T17:30:40+09:00
- **完了日時**: 2026-04-19T17:31:59+09:00

## タスク概要

`kanban/` ディレクトリにマークダウンファイルを追加するスキルを作成して欲しい。

また自動的に番号を採番して欲しい。

引数としてタイトルは想定するが、必須とはしない。

ファイル名で使用するタイトルは英語、ファイル内のタイトルは日本語にして欲しい。

要望と目的を必須ではないが、受け取るようにして欲しい。
またタイトルを指定していない場合は、考えてつけて欲しい。

また、マークダウンファイル作成後、 `/kanban` を実行するかを聞いて欲しい。

## 調査結果

### `bin/add_kanban.sh`

```bash
#!/bin/bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $(basename "$0") <title>" >&2
    exit 1
fi

title="$1"
kanban_dir="$(cd "$(dirname "$0")/.." && pwd)/kanban"

# 既存ファイルから最大番号を取得
max_num=-1
for f in "$kanban_dir"/[0-9][0-9][0-9][0-9]_*.md; do
    [ -e "$f" ] || continue
    num="${f##*/}"
    num="${num%%_*}"
    num=$((10#$num))
    if [ "$num" -gt "$max_num" ]; then
        max_num="$num"
    fi
done

next_num=$((max_num + 1))
padded=$(printf "%04d" "$next_num")
filename="${kanban_dir}/${padded}_${title}.md"

touch "$filename"
echo "Created: $filename"
```

- 引数: タイトル1つ（必須、シェル引数1つのみ）
- 動作: `kanban/` の `[0-9][0-9][0-9][0-9]_*.md` から最大番号を抽出 → +1 → 4桁0パディング → `{xxxx}_{title}.md` を `touch` で生成（本文は空）
- このスクリプトは CLI 用途で残す。スキルでは使わない。

### `.claude/skills/kanban/SKILL.md`

フロントマター:
```
---
name: kanban
description: プロジェクトの kanban タスクを実行する。kanban/ ディレクトリのタスクファイル（目的と要望が書かれたマークダウン）に基づき、まずプランモードで計画を立てて承認を得た後、logs/ に詳細な作業ログを残しながら実装する。ユーザーが /kanban を呼び出したとき、あるいは「kanban タスクを進める」「0001 を実行」のようにタスク番号やファイル名を指定して kanban 作業の開始を求めたときに使用する。
---
```

本文に日本語で手順を記述する構造（## 引数、## フェーズ1、## フェーズ2、## 注意事項 セクション）。

### `.claude/skills/kanban/references/kanban-workflow.md`

kanban ファイルの規約:
- ファイル名: `{xxxx}_{title}.md` — `xxxx` は4桁0パディング連番、`title` は英語
- kanban と logs で同じファイル名
- 基本構造:
  ```markdown
  # タイトル
  ## 目的
  （Why — 必須: /kanban 実行時に存在確認される）

  ## 要望
  （What/How）
  ```
- タイムスタンプ: `TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"`

### 既存 kanban ファイル命名実態

Glob 結果（抜粋）:
- `kanban/0028_make_kanban_skill.md`
- `kanban/0029_create_add_kanban_skill.md`

実態は全て snake_case。kanban-workflow.md は「ハイフン区切り推奨」と書いているが既存と合わせて snake_case を採用。

### 既存 kanban 最大番号

`kanban/0029_create_add_kanban_skill.md` が現在最大。

## 実装プラン

### 作成するファイル

`.claude/skills/add-kanban/SKILL.md` のみ新規作成。

### スキルの動作フロー

1. **args 解析**: 自由形式テキストから英語タイトル（ファイル名用）・日本語タイトル（本文見出し用）・要望（What/How）・目的（Why）を抽出する。すべて任意。タイトル未指定時は Claude が内容から考案する。
2. **採番**: Glob で `kanban/[0-9][0-9][0-9][0-9]_*.md` を取得し、最大番号 +1 を4桁0パディング。
3. **ファイル生成**: Write ツールで以下テンプレートで作成:
   ```markdown
   # {日本語タイトル}

   ## 目的
   {目的 — 未指定なら空行のみ}

   ## 要望
   {要望 — 未指定なら空行のみ}
   ```
4. **報告**: 作成したファイルパスをユーザーに報告。目的が空の場合は「/kanban 実行前に目的セクションへの記入が必要」と注意喚起。
5. **確認**: AskUserQuestion で「続けて `/kanban` を実行するか」確認。
6. **起動**: Yes の場合、Skill で `kanban` スキルを `args: "{xxxx}"` 指定で起動。

### 設計判断

- **自前採番 + Write**: `add_kanban.sh` 呼び出しは不採用。`touch` 後に Write で上書きする二重操作より、Glob + Write で直接完結させる方がシンプル。
- **ファイル名は snake_case**: 既存ファイルと揃える。
- **要望・目的が未指定でも空欄で作成**: 軽く使えるようにする。ただし目的が空のまま `/kanban` に進むと弾かれる点を注意喚起。
- **`/kanban` 実行確認は AskUserQuestion**: 選択肢の方がモバイルでも操作しやすい。

### 作業範囲外

- `bin/add_kanban.sh` は変更しない（CLI 用途で残す）
- `.claude/skills/kanban/` 配下は変更しない

## プランニング経緯

### 初回提案

上記プランをそのまま提案。

### ユーザーフィードバック

初回提案がそのまま承認された。

## 会話内容

### [17:27] ユーザー指示（1回目）
/kanban 呼び出し。タスクファイルに目的セクションがなかったため、Claude が目的セクションの追加を求めた。

### [17:29] ユーザー指示（2回目）
/kanban 再呼び出し。目的セクション「Claude Codeのインタラクティブセッションでカンバンを追加できるようにしたい。毎回インタラクティブセッションを抜けてから `add_kanban.sh` するのは面倒なので。」が追加されていた。

### [17:30] Claude 調査・プラン提示

既存の `bin/add_kanban.sh`、`.claude/skills/kanban/SKILL.md`、`references/kanban-workflow.md`、既存 kanban ファイル一覧を調査。

プランモードに入り、`.claude/skills/add-kanban/SKILL.md` を新規作成するプランを提示。採番・ファイル生成・`/kanban` 実行確認の流れを設計。

### [17:30] ユーザー承認

プランが承認された。実装フェーズへ移行。

## 編集したファイル

| ファイル | 変更内容 |
|---------|---------|
| `.claude/skills/add-kanban/SKILL.md` | 新規作成 |
| `kanban/0029_create_add_kanban_skill.md` | `## プラン` と `## 完了サマリー` を追記 |

## 実行したコマンド

```bash
TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"
```

## 判断・意思決定

- Glob + Write で直接ファイル生成（`add_kanban.sh` 呼び出し不採用）
- ファイル名は snake_case（既存と合わせる）
- 要望・目的未指定でも空欄で作成（注意喚起付き）
- `/kanban` 実行確認に AskUserQuestion を使用

## エラー・問題

（なし）
