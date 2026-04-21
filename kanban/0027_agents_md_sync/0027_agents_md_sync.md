# CLAUDE.mdの変更をAGENTS.mdに追従させる仕組みが動いていない
## 要望
CLAUDE.mdを更新したのにAGENTS.mdに追従させる仕組みが動かなかった。
修正して欲しい。

## 目的
`kanban/`, `logs/` 両ディレクトリについてはレビューの対象外にするためCLAUDE.mdにその旨を追記した。

しかし、一番重要なAGENTS.mdへの同期が行われなかった。
AGENTS.mdに同期されないと、Codex Reviewで反映されない。

もともとCLAUDE.mdの更新を自動的に追随するための仕組みとして作ったのに動かないのは困る。

## プラン

`.claude/hooks/sync-agents-md.sh` のホワイトリスト（SHARED_SECTIONS）をブラックリスト（EXCLUDE_SECTIONS）に変換する。

- CLAUDE.md の全 `##` セクションを AGENTS.md に同期（除外リスト対象を除く）
- AGENTS.md 固有セクション（Review guidelines）は末尾に保持
- 修正後のスクリプトを手動実行して AGENTS.md を即時再生成

---

## 完了サマリー

- **完了日時**: 2026-04-19T16:57:19+09:00
- **対応内容**:
  - `.claude/hooks/sync-agents-md.sh` のホワイトリスト (`SHARED_SECTIONS`) をブラックリスト (`EXCLUDE_SECTIONS=()`) 方式に変更
  - CLAUDE.md の全セクションが自動的に AGENTS.md に同期されるようになった
  - スクリプト手動実行で AGENTS.md を再生成し、「CI」「Kanban ワークフロー」セクションを追加
- **変更ファイル**:
  - `.claude/hooks/sync-agents-md.sh`
  - `AGENTS.md`
- **備考**: EXCLUDE_SECTIONS は現時点では空。将来 Claude Code 固有セクションを除外したい場合はこのリストに追加する。
