# 調査依頼: Codex Code Reviewがスキップされた
## 内容
現在作業中のブランチをpushしたところ、Codex Code Reviewがスキップされた。
その理由を調べて欲しい。

また簡単に解決できるのなら修正して欲しい。

## プラン

- `.github/workflows/codex-code-review.yml` の `if: github.event.pull_request.draft == false` 条件により、ドラフト PR ではジョブがスキップされる
- PR #2 がドラフト状態であることが原因と特定
- `gh pr ready 2` を実行してドラフトを解除する

## 完了サマリー

完了日時: 2026-04-13T13:47:40+09:00

**原因**: `.github/workflows/codex-code-review.yml` の 13 行目にある `if: github.event.pull_request.draft == false` 条件により、PR #2 がドラフト状態の間はジョブがスキップされていた。

**対応**: `gh pr ready 2` を実行し、PR #2 を「Ready for review」に変更した。これにより `ready_for_review` イベントが発火し、Codex Code Review が実行される。ワークフロー自体の変更は不要（ドラフトスキップは意図した設計）。