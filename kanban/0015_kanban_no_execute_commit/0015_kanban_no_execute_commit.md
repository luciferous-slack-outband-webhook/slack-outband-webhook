# `/kanban`コマンドで `/commit`を自動実行しないようにしたい
## 要望
`/kanban`を実行したときに `/commit`を自動で実行しようとした。
これをやらないようにして欲しい。

## プラン

- `.claude/commands/kanban.md` のフェーズ2の末尾に注意事項セクションを追加する
- 「git commit や `/commit` コマンドを自動実行しないこと」を明記する
- kanban コマンド定義自体には commit 実行の指示は無いが、Claude が自主判断で実行してしまうため、明示的な禁止指示で防止する

---

## 完了サマリー

- **完了日時**: 2026-04-13T14:06:43+09:00
- **対応内容**:
  - `.claude/commands/kanban.md` に「## 注意事項」セクションを追加
  - `git commit や /commit コマンドを自動実行しないこと` を明記
- **変更ファイル**:
  - `.claude/commands/kanban.md`