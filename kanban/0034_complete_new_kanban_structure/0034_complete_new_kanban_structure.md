# kanban 新構造への完全対応

## 目的

新構造への適用を完全なものにしたい。

## 要望

既存タスクを新構造に移行しました。スキルやワークフローなど新構造への対応が終わっていないものがあれば対応してください。

## プラン

調査の結果、対応が必要な残タスクは2件（いずれも軽微）:

1. **`.github/workflows/codex-code-review.yml` L35 ステップ名**
   - `Strip kanban and logs from workspace` → `Strip kanban from workspace` に改名
   - 実処理は 0033 で既に `rm -rf kanban` のみになっているが、名前だけ古いまま残っていた
2. **`kanban/.gitkeep` の削除**
   - 35 件のタスクディレクトリが常に存在するため、空ディレクトリ維持用の `.gitkeep` は不要になった
   - `git rm` で削除

他のスクリプト・スキル・hook・ワークフロー・ドキュメントは全て新構造対応済み。`logs/` ディレクトリも既に削除済みであることを確認。

## 完了サマリー

- **完了日時**: 2026-04-21T21:18:22+09:00
- **対応内容**:
  - `.github/workflows/codex-code-review.yml` L35 ステップ名を `Strip kanban from workspace` に改名（実処理との不一致を解消）
  - `kanban/.gitkeep` を `git rm` で削除（新構造下では常にサブディレクトリが存在するため不要）
  - リポジトリ全体を徹底調査し、他に対応漏れがないことを確認
- **変更ファイル**:
  - `.github/workflows/codex-code-review.yml`
  - `kanban/.gitkeep`（削除）
