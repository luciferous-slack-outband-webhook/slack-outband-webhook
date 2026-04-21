# 0026_codex_review_update ログ

## ヘッダー

- タスク番号: 0026
- タイトル: Codex Reviewにおいて `kanban/` と `logs/` がレビュー対象から外れていない
- 開始時刻: 2026-04-19T16:43:42+09:00
- 完了時刻: 2026-04-19T16:45:08+09:00

## タスク概要

`kanban/` と `logs/` がまだレビュー対象になっているのでレビュー対象外にして欲しい。

## 調査結果

### ファイル調査

#### `.github/workflows/codex-code-review.yml`

ワークフローファイルは 269 行。主要なステップ構成：
1. `Checkout pull request merge commit` - PR のマージコミットをチェックアウト
2. `Fetch base and head refs` - base/head を fetch
3. `Fetch past review comments` - GitHub API で過去コメントを収集し `past-review-context.md` に書き込む
4. `Build review prompt` - コピーしたプロンプトに PR 情報・過去コメント・git diff を追記してプロンプトを構築
5. `Run Codex structured review` - Codex action を sandbox: read-only で実行
6. `Post inline review comments` - 出力 JSON をパースして PR review を投稿

前回コミット `4d7f6ff` で `Build review prompt` の 3 つの `git diff` コマンドに pathspec 除外 `':(exclude)kanban/'` `':(exclude)logs/'` を付与済み（105-110 行目）。

`Fetch past review comments` ステップのインラインコメント取得（60-62 行目）は `.path` フィルタなしで全コメントを取得する:
```bash
gh api "repos/${REPOSITORY}/pulls/${PR_NUMBER}/comments" \
  --jq '.[] | "- **\(.user.login)** (\(.created_at)) [`\(.path)` L\(.line // .original_line // "?")`]:\n  \(.body)\n"' \
  >> "$PAST_CONTEXT" 2>/dev/null || echo "_取得できませんでした_" >> "$PAST_CONTEXT"
```

#### `.github/codex/codex-prompt.md`

34 行。Codex へのレビュー指示文。`kanban/` や `logs/` を無視する指示は含まれていない。

#### `kanban/0026_codex_review_update.md` (タスクファイル)

2601 行。末尾付近（2408 行目）に PR#17 で実行された Codex Review の `codex-output.json` の内容が含まれていた。4 件の findings は以下のファイルを参照していた:
- `kanban/0022_check_slack_signing.md`（kanbanに目的セクションがない）
- `logs/0022_check_slack_signing.md`（プランニング経緯・会話内容セクションが欠落）
- `logs/0023_codex_review_update.md`（プランニング経緯がテンプレート要件を満たしていない）
- `logs/0024_fix_clippy.md`（同上）

### 根本原因の特定

PR#17 の Codex Review の実行ログ（2127 行目以降）に以下のコマンド実行記録があった:
```
/bin/bash -lc 'ls kanban logs'
/bin/bash -lc 'nl -ba kanban/0022_check_slack_signing.md'
/bin/bash -lc 'nl -ba logs/0022_check_slack_signing.md'
/bin/bash -lc 'nl -ba logs/0023_codex_review_update.md'
/bin/bash -lc 'nl -ba logs/0024_fix_clippy.md'
```

Codex action の sandbox は `read-only` だが、`actions/checkout` でチェックアウトされたリポジトリ全体を読める。Codex は diff に含まれていないファイルでも、ファイルシステムを直接探索して読み込んでいた。したがって、`git diff` の pathspec 除外だけでは不十分だった。

また、`Fetch past review comments` ステップで取得するインラインコメントに path フィルタがないため、以前のCodex Review が `kanban/` や `logs/` にコメントを残していた場合、次回プロンプトに持ち込まれ続けるという副次的な問題も存在する（今回のPR#17での実行では、過去コメントが `kanban/` に付いていなかったため影響はなかったが、将来的に問題になり得る）。

## 実装プラン

### 修正ファイル
- `.github/workflows/codex-code-review.yml`
- `CLAUDE.md`

### 修正内容

#### 1. チェックアウト後に `kanban/` と `logs/` を削除

`Checkout pull request merge commit` ステップの直後に新ステップを追加する:
```yaml
- name: Strip kanban and logs from workspace
  run: |
    set -euxo pipefail
    rm -rf kanban logs
```

ワーキングツリーから実ファイルを削除するだけで `.git` インデックスは保持されるため、以降の `git diff BASE_SHA HEAD_SHA` は引き続き正しく動く（diff はインデックスとコミット履歴を参照する）。既存の pathspec 除外は二重の安全策として残す。

#### 2. 過去インラインコメントから `kanban/` と `logs/` を除外

`Fetch past review comments` ステップの `gh api` 呼び出しの `--jq` フィルタに path フィルタを追加する:
```bash
gh api "repos/${REPOSITORY}/pulls/${PR_NUMBER}/comments" \
  --jq '.[] | select((.path // "") | startswith("kanban/") or startswith("logs/") | not) | "- ...' \
  >> "$PAST_CONTEXT" 2>/dev/null || echo "_取得できませんでした_" >> "$PAST_CONTEXT"
```

`select(... | not)` で `.path` が `kanban/` または `logs/` で始まるコメントを除外する。`.path // ""` は path が無いコメントへの安全策。

一般コメント・レビュー判定履歴は `.path` を持たないためフィルタ対象外。

#### 3. CLAUDE.md に除外方針を明記

`## Kanban ワークフロー` セクションに箇条書き 1 行追加:
```
- `kanban/` と `logs/` は開発者向けの記録であり、Codex Code Review の対象外としている（`.github/workflows/codex-code-review.yml` で作業ツリーから削除している）
```

## プランニング経緯

### 最初に提示したプランの概要

1. ワークフローで `kanban/` と `logs/` を削除するステップを追加
2. 過去インラインコメントから `kanban/` と `logs/` を除外

### ユーザーのフィードバック

ユーザーから「CLAUDE.md においても `kanban/` と `logs/` についてはCodexのレビュー対象外にすることを明記して欲しい」というフィードバックがあり、CLAUDE.md の修正をプランに追加した。

### 最終プランへの変更内容

CLAUDE.md の `## Kanban ワークフロー` セクションへの 1 行追加を修正スコープに加えた。

## 会話内容

- ユーザーが `/kanban 0026` を実行
- Claude がタスクファイル・ワークフローファイル・プロンプトファイル・Codex の実行ログを調査
- advisor を呼んで確認 → 「過去コメント取得ステップが path フィルタなし」「Codex がファイルシステムを直接読んでいる可能性」を指摘
- さらに調査して PR#17 の codex-output.json と実行ログを確認 → Codex がファイルシステムを直接読んでいたことを確証
- プランモードに入り、「ワークフローへの削除ステップ追加」「過去コメントフィルタリング」のプランを提示
- ユーザーが「CLAUDE.md にも明記してほしい」とフィードバック
- プランに CLAUDE.md 修正を追加して再提示 → 承認

## 編集したファイル

- `.github/workflows/codex-code-review.yml`
  - `Checkout pull request merge commit` の直後に `Strip kanban and logs from workspace` ステップを追加（`rm -rf kanban logs`）
  - `Fetch past review comments` の `gh api .../pulls/${PR_NUMBER}/comments` の `--jq` フィルタに `select((.path // "") | startswith("kanban/") or startswith("logs/") | not)` を追加
- `CLAUDE.md`
  - `## Kanban ワークフロー` セクションに `kanban/` と `logs/` が Codex Code Review の対象外である旨を 1 行追加
- `kanban/0026_codex_review_update.md`
  - `## プラン` セクションを追加
  - `## 完了サマリー` を追加（後述）
- `logs/0026_codex_review_update.md`
  - このファイルを新規作成

## 実行したコマンド

- `TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"` - 開始・完了時刻取得

## 判断・意思決定

- **`rm -rf kanban logs` を削除ステップとして追加**: Codex action の sandbox は read-only だがリポジトリ全体を読めるため、ワーキングツリーからファイルを削除することが唯一の確実な対策。
- **既存の pathspec 除外は残す**: 二重の安全策として、`git diff` の `:(exclude)kanban/` と `:(exclude)logs/` はそのまま残した。
- **過去コメントの path フィルタ**: 将来 kanban/logs に Codex がコメントを残した場合に次回プロンプトに持ち込まれないよう、`select()` でフィルタリングを追加した。

## エラー・問題

特になし。
