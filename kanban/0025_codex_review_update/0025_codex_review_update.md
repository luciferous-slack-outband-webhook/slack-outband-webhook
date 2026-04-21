# Codex Reviewからkanban/とlogs/を対象外にする
## 要望
Codex Reviewにおいて、 `kanban/` と `logs/` の両ディレクトリをレビューの対象外とする

## 理由
`kanban/` と `logs/` はあくまで開発のまとめ用のもので、この中身をレビューして欲しいとは思っていない。
そのためレビュー対象から外して欲しい。

## プラン

`.github/workflows/codex-code-review.yml` の `Build review prompt` ステップにある 3 つの `git diff` コマンドに pathspec 除外指定 `-- '.' ':(exclude)kanban/' ':(exclude)logs/'` を付与する。

## 完了サマリー

完了日時: 2026-04-19T16:20:50+09:00

`.github/workflows/codex-code-review.yml` の `Build review prompt` ステップにある 3 つの `git diff` コマンドそれぞれに `-- '.' ':(exclude)kanban/' ':(exclude)logs/'` を追加した。これにより `kanban/` と `logs/` の変更は Codex のレビュー対象から除外される。