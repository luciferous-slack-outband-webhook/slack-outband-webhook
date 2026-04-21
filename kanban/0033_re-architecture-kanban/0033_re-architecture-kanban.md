# `/kanban` スキルで使うディレクトリ構成を再構築
## 要望

- `./kanban/`
  - `{xxxx}_{title}/`
    - `{title}.md`
    - `log.md`
 

上記ディレクトリ構造に変更する。
`xxxx`や`title`などは今と同様にする。
また、ファイルのディレクトリ構成が変わるだけで仕組みは変えない。

`/add-kanban` でも `./kanban/{xxxx}_{title}/{title}.md` を作成する。

## 目的

カンバンとログが離れているので読みにくかった。
これを解消したい。

## プラン

- 対象: skill・hook・スクリプト・ドキュメント・CI ワークフローを新方式のみ対応に変更
- 既存 0000〜0032 の移行はユーザーが手動 `git mv` で行う（Claude Code 実行時に rename が delete+create になる過去事例のため）
- 本タスク (0033) 自身も実装中は旧位置のまま進め、完了後にユーザーが移行

新構造:
- `kanban/{xxxx}_{title}/{xxxx}_{title}.md` — タスクファイル（ディレクトリ名と同じ basename）
- `kanban/{xxxx}_{title}/log.md` — ログファイル（固定名）

変更ファイル:
- `.claude/skills/kanban/SKILL.md` — ログパス・自動選択ロジックの記述を新方式に
- `.claude/skills/kanban/references/kanban-workflow.md` — 命名規則・テンプレート全面改訂
- `.claude/skills/add-kanban/SKILL.md` — 採番 glob・ファイル作成先を新方式に
- `bin/add_kanban.sh` — 採番・ファイル作成処理を新方式に
- `.claude/hooks/check-kanban.sh` — 新構造のディレクトリを走査するように
- `CLAUDE.md` — ディレクトリ構造説明を新方式に（AGENTS.md は sync hook で自動同期）
- `.github/workflows/codex-code-review.yml` — `logs/` への参照を削除

## 完了サマリー

- **完了日時**: 2026-04-21T20:56:24+09:00
- **対応内容**:
  - skill・hook・スクリプト・CI ワークフローを新構造 `kanban/{xxxx}_{title}/{xxxx}_{title}.md` + `kanban/{xxxx}_{title}/log.md` に対応
  - 旧構造 `kanban/{xxxx}_{title}.md` / `logs/{xxxx}_{title}.md` への参照をすべて削除
  - 動作確認済み（check-kanban.sh: 新構造の未完了検出・完了済み非検出）
- **変更ファイル**:
  - `.claude/skills/kanban/SKILL.md`
  - `.claude/skills/kanban/references/kanban-workflow.md`
  - `.claude/skills/add-kanban/SKILL.md`
  - `bin/add_kanban.sh`
  - `.claude/hooks/check-kanban.sh`
  - `CLAUDE.md`（AGENTS.md は hook で自動同期済み）
  - `.github/workflows/codex-code-review.yml`
- **備考**: 既存 0000〜0032 および本タスク 0033 の移行はユーザーが手動 `git mv` で行う（下記参照）

---

### 手動移行コマンド（ユーザー向け）

各タスクを新構造に移行するには以下を実行してください（例: 0033）:

```bash
# ディレクトリを作成して git mv
mkdir -p kanban/0033_re-architecture-kanban
git mv kanban/0033_re-architecture-kanban.md kanban/0033_re-architecture-kanban/0033_re-architecture-kanban.md
git mv logs/0033_re-architecture-kanban.md kanban/0033_re-architecture-kanban/log.md
```

全件一括移行スクリプト例（kanban/ 直下の `.md` を対象）:

```bash
for f in kanban/[0-9][0-9][0-9][0-9]_*.md; do
    base="$(basename "$f" .md)"
    mkdir -p "kanban/${base}"
    git mv "$f" "kanban/${base}/${base}.md"
    log="logs/${base}.md"
    [ -f "$log" ] && git mv "$log" "kanban/${base}/log.md"
done
```