# 0036_kanban_session_name_format 作業ログ

## 基本情報

- **タスクファイル**: kanban/0036_kanban_session_name_format/0036_kanban_session_name_format.md
- **開始日時**: 2026-04-21T22:04:24+09:00
- **完了日時**: 2026-04-21T22:04:24+09:00

## タスク概要

`/kanban` スキルにおいてセッション名を `{xxxx}_{title}` にして欲しい。

## 調査結果

### CLI リファレンス（https://code.claude.com/docs/en/cli-reference）

`--name, -n` フラグ:
> Set a display name for the session, shown in `/resume` and the terminal title. You can resume a named session with `claude --resume <name>`. `/rename` changes the name mid-session and also shows it on the prompt bar.

これはセッション**起動時**にのみ有効なフラグで、既存のセッションに対して途中から適用する手段ではない。

### コマンドリファレンス（https://code.claude.com/docs/en/commands）

`/rename [name]` コマンドの説明:
> Rename the current session and show the name on the prompt bar. Without a name, auto-generates one from conversation history

つまり `/rename` はミッドセッションでセッション名を変更できる。引数なしの場合は会話履歴から自動生成される。

### スキルリファレンス（https://code.claude.com/docs/en/slash-commands → スキルのページ）

スキルドキュメントの記述:
> A few built-in commands are also available through the Skill tool, including `/init`, `/review`, and `/security-review`. Other built-in commands such as `/compact` are not.

Skill ツールから呼び出せる組み込みコマンドのホワイトリストは `/init`, `/review`, `/security-review` のみ。`/rename` はこのリストに含まれていない。

### その他の検討事項

- `printf '\033]0;...\007'` でターミナルエミュレータのタブタイトルを変更することは技術的に可能。ただしこれは Claude Code の内部セッション名（`/resume` 一覧に表示されるもの）とは別チャネルであり、要求を満たさない。
- ユーザーが「`/kanban` 実行時に Claude Code がセッション名を自動で付けている」と観察していた現象は、Claude Code の会話履歴ベースの自動命名 heuristic（`/rename` 引数なし実行と同様の仕組み）によるものと推測される。アシスタント側から決定的に制御する手段はない。

## 実装プラン

### 検討した代替案と却下理由

1. **`/kanban` スキルで `/rename {xxxx}_{title}` を自動実行する**  
   → スキル/アシスタントから `/rename` を呼び出す手段がない（ホワイトリスト外）。却下。

2. **`claude --name {xxxx}_{title}` で新規セッション起動を運用にする**  
   → セッション起動前の作業（既存セッション内では効果なし）。また既存のセッションを壊す必要がある。ユーザーは「自動化できない理由はないはず」との見解だったため、この workaround に対しては「実現不可と判断してクローズ」を選択。

3. **ターミナルタブタイトルの変更のみ行う**  
   → `/resume` 一覧（Claude Code 内部セッション名）とは別物。要求を満たさない。却下。

### 採用アプローチ

タスクを「実現不可」としてクローズし、ドキュメントを残す。

## プランニング経緯

### 初回提案

以下 4 択を提示:
1. `/kanban` が `/rename` の実行を促すメッセージを出力する
2. kanban 専用の起動方法 (`--name`) をドキュメント化する
3. `printf` でターミナルタブタイトルのみ変更する
4. 方法 1 と 2 を組み合わせる

### ユーザーフィードバック（1 回目）

「セッション名が指す対象は Claude Code 内部の識別名（`/rename` で指定できる領域）。`/kanban` 実行時に Claude Code が kanban 内容を読んで自動で付けているように見える。セッション名変更機能があるのだから自動化できないはずがない」との追加情報・意見が寄せられた。

### ユーザーフィードバック（2 回目）

最終的な 3 択（`/kanban` でメッセージ、`--name` 運用ドキュメント化、実現不可としてクローズ）に対し、「実現不可と判断しタスクをクローズする」を選択。

### 最終プラン

kanban ファイルに完了サマリー（実現不可・調査結果記録）を追記し、ログを残してクローズ。

## 会話内容

### [22:00 頃] ユーザー指示（`/add-kanban` 経由）

要望: `/kanban` スキルにおいてセッション名を `{xxxx}_{title}` にして欲しい。  
目的: セッション名と対象のカンバンが一致するようにして認知負荷を下げたい。

### [22:00 頃] Claude 対応

`/add-kanban` スキルが `0036_kanban_session_name_format` タスクファイルを作成し、続けて `/kanban` スキルを起動。

### [22:01 頃] プランニング開始

claude-code-guide エージェントへ調査を依頼。エージェントは「`claude --name` フラグと `/rename` コマンドが使える」と回答（出典に `docs.claude.com` リンクを提示）。ただし「スキルから自動実行できるか」については明示なし。

### [22:01 頃] 公式ドキュメント直接確認

`--name` フラグと `/rename` コマンドの存在を確認。スキルドキュメントにて Skill ツールから呼べる組み込みコマンドは `/init` / `/review` / `/security-review` のホワイトリストのみであり、`/rename` は含まれないことを確認。

### [22:02 頃] advisor によるレビュー

「`--name` はセッション起動後には使えない。`/rename` はユーザーが入力する必要がある。スキルから自動実行する手段はない」と指摘。ユーザーに選択肢を提示するよう推奨。

### [22:03 頃] 最初の選択肢提示

4 択を AskUserQuestion で提示したが、ユーザーが質問を拒否し「セッション名の対象を明確にしたい」との追加情報を入力。

### [22:03 頃] ユーザーの追加情報

「Claude Code 内部の識別名を指す。`/kanban` 実行時に自動でセッション名が付いているのでスキルから自動化できるはず」との見解。

### [22:04 頃] 再調査

`/rename` のドキュメント詳細を確認。「引数なしで会話履歴から自動生成」との記述を発見し、ユーザーの観察（自動命名）の原因が heuristic 自動命名であることを確認。スキルから決定的に制御する手段がないことを改めて確認。

### [22:04 頃] 最終選択肢提示

「実現不可としてクローズ」を含む 3 択を提示。ユーザーが「実現不可と判断しクローズ」を選択。

## 編集したファイル

| ファイル | 変更内容 |
|---------|---------|
| `kanban/0036_kanban_session_name_format/log.md` | 新規作成 |
| `kanban/0036_kanban_session_name_format/0036_kanban_session_name_format.md` | `## 完了サマリー` 追記 |

## 実行したコマンド

```bash
TZ=Asia/Tokyo date +"%Y-%m-%dT%H:%M:%S+09:00"
```

## 判断・意思決定

- Claude Code の公式ドキュメントを直接確認し、Skill ツールから呼べる組み込みコマンドのホワイトリストに `/rename` が含まれないことを確認。
- ユーザーの「自動化できないはずがない」という見解に対し、公式ドキュメントの根拠を示して丁寧に説明した。
- 最終的にユーザー自身が「実現不可」と判断しクローズを選択。

## エラー・問題

- なし
