# add shell script
## 要望

`bin`ディレクトリを作成し、 `add_kanban.sh`というシェルスクリプトを書いてください。

引数でtitleを受け取り、 `kanban/xxxx_title.md`という空のファイルを作成します。
xxxxは自動で採番してください。

また、.envrcを作成しパスに `bin` ディレクトリを追加してください。

## プラン

### 1. `bin/add_kanban.sh` の作成
- `bin/` ディレクトリを新規作成
- `add_kanban.sh` を作成し、実行権限を付与
- スクリプト仕様:
  - 引数: `title`（必須、1つ）。未指定時はエラー終了
  - `kanban/` 内の既存ファイルから最大番号を取得し +1 で採番（4桁ゼロ埋め）
  - ファイルがない場合は `0000` から開始
  - `kanban/{xxxx}_{title}.md` を空ファイルとして作成

### 2. `.envrc` の作成
- `PATH_add bin` で `bin/` を PATH に追加（direnv 標準関数）

### 3. `.gitignore` の確認
- `.envrc` が gitignore 対象でないか確認し、必要に応じて対応

## 完了サマリー

- 完了日時: 2026-04-12T01:33:12+09:00
- 作成ファイル:
  - `bin/add_kanban.sh`: 引数 `title` を受け取り `kanban/{xxxx}_{title}.md` を自動採番して作成するスクリプト
  - `.envrc`: `PATH_add bin` で `bin/` を PATH に追加（direnv 用）
- `.gitignore` に `.envrc` の記載がないため、そのままコミット対象として扱った