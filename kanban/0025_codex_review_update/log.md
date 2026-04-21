# 0025_codex_review_update ログ

開始日時: 2026-04-19T16:20:13+09:00

## タスク概要

Codex Review において、`kanban/` と `logs/` の両ディレクトリをレビューの対象外とする。

理由: `kanban/` と `logs/` はあくまで開発のまとめ用のもので、この中身をレビューして欲しいとは思っていない。そのためレビュー対象から外して欲しい。

## 調査結果

### `.github/workflows/codex-code-review.yml`

`Build review prompt` ステップ（行82-111）で、以下 3 つの `git diff` コマンドにより差分情報を収集し、Codex プロンプトに埋め込んでいる:

```bash
# 変更ファイル一覧
git --no-pager diff --name-status "${BASE_SHA}" "${HEAD_SHA}"

# 差分統計
git --no-pager diff --unified=5 --stat=200 "${BASE_SHA}" "${HEAD_SHA}"

# 差分本体
git --no-pager diff --unified=5 "${BASE_SHA}" "${HEAD_SHA}"
```

これら 3 コマンドは全パスを対象とするため、`kanban/` と `logs/` 配下の変更も Codex に渡ってしまう。

### `.github/codex/codex-prompt.md`

Codex へのレビュー指示プロンプト。差分全体に対してレビューを行う方針が記述されている。プロンプト側で「無視して」と書くだけでは LLM が従う保証が弱く、トークン消費も無駄なため、ワークフロー側の `git diff` コマンドで除外するアプローチを採用する。

## 実装プラン

`.github/workflows/codex-code-review.yml` の `Build review prompt` ステップにある 3 つの `git diff` コマンドそれぞれに pathspec 除外指定を付与する。

```bash
# 変更後
git --no-pager diff --name-status "${BASE_SHA}" "${HEAD_SHA}" -- '.' ':(exclude)kanban/' ':(exclude)logs/'
git --no-pager diff --unified=5 --stat=200 "${BASE_SHA}" "${HEAD_SHA}" -- '.' ':(exclude)kanban/' ':(exclude)logs/'
git --no-pager diff --unified=5 "${BASE_SHA}" "${HEAD_SHA}" -- '.' ':(exclude)kanban/' ':(exclude)logs/'
```

`'.'` を先頭に置く理由: `git diff` の pathspec は除外のみを指定すると何もマッチしないため、包含 pathspec を先に置く。
`:(exclude)` 長形式を採用: `:!` 短形式より可読性が高い。
`--name-status` も除外: 「変更ファイル一覧」に `kanban/` が載っていると Codex が気にしてしまう可能性があるため。

## プランニング経緯

初回提案がそのまま承認された。

## 会話内容

- ユーザー: `/kanban` コマンド実行
- Claude: kanban/0025_codex_review_update.md を読み込み、コードベース調査（ワークフローファイル・プロンプトファイルを確認）
- Claude: プランモードでプランを提示（git diff に pathspec 除外を追加するアプローチ）
- ユーザー: プランを承認

## 編集したファイル

- `.github/workflows/codex-code-review.yml` — `Build review prompt` ステップの git diff 3 コマンドに `-- '.' ':(exclude)kanban/' ':(exclude)logs/'` を追加

## 実行したコマンド

（なし）

## 判断・意思決定

- ワークフロー側（`git diff` コマンド）で除外することを選択。プロンプト側での除外指示は LLM の保証が弱くトークン消費も無駄なため。
- `:(exclude)` 長形式を採用（可読性重視）。
- `'.'` を先頭に付けた（除外のみ指定では何もマッチしないため）。

## エラー・問題

（なし）

完了日時: 2026-04-19T16:20:50+09:00
