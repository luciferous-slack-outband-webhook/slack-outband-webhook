# add shell script 作業ログ

## 基本情報

- タスク: 0005_add_shell_scritp
- 開始日時: 2026-04-12T01:32:43+09:00
- 完了日時: 2026-04-12T01:33:12+09:00

## 作業記録

### ステップ1: コードベース調査

- `kanban/` ディレクトリの命名パターン確認: `{xxxx}_{title}.md`（4桁ゼロ埋め連番）
- `bin/` ディレクトリ: 未存在
- `.envrc`: 未存在
- `.gitignore`: `.envrc` の記載なし → コミット対象として扱う

### ステップ2: `bin/add_kanban.sh` 作成

- `bin/add_kanban.sh` を新規作成
- `set -euo pipefail` でエラー時即終了
- 引数未指定時はエラーメッセージを出力して exit 1
- `kanban/` 内の `[0-9][0-9][0-9][0-9]_*.md` パターンから最大番号を取得し +1 で採番
- `printf "%04d"` でゼロ埋め4桁にフォーマット
- `touch` で空ファイルを作成
- `chmod +x` で実行権限付与

### ステップ3: `.envrc` 作成

- `PATH_add bin` を記載（direnv の標準関数）
- `.gitignore` に `.envrc` の記載なし → コミット対象

### ステップ4: 動作確認

- `bash bin/add_kanban.sh test_task` → `kanban/0006_test_task.md` が作成されることを確認
- 引数なしで `bash bin/add_kanban.sh` → Usage エラーが出ることを確認
- テストファイル `kanban/0006_test_task.md` を削除

## 編集ファイル

- `bin/add_kanban.sh`（新規）
- `.envrc`（新規）

## 実行コマンド

```bash
chmod +x bin/add_kanban.sh
bash bin/add_kanban.sh test_task  # 動作確認
rm kanban/0006_test_task.md       # テストファイル削除
```

## 判断事項

- `.envrc` はチームで共有するプロジェクト設定のため、gitignore に追加せずコミット対象とした
