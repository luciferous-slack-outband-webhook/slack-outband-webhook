# 0016 Codex Code Review エラー修正

## 開始時刻
2026-04-13T14:17:27+09:00

## タスク概要
Codex Code Review で エラー終了した。修正して欲しい。

## 調査結果

### 調査したファイル・情報
- `.github/workflows/codex-code-review.yml`
- `kanban/0016_error_fix_codex_code_review.md`（GitHub Actions ログ）

### 現状の構造
- ワークフロー: PR の push 時に `openai/codex-action@v1` を使って Codex Code Review を実行
- エラー: `Error: Failed to read server info from /home/runner/.codex/24326152740.json`
- 約60秒間ポーリングを繰り返してタイムアウトで失敗

### 発見した事実
GitHub Actions ログの env ブロックに `OPENAI_API_KEY: ` が空文字表示（シークレットが設定されている場合は `***` と表示される）。

**根本原因**: `OPENAI_API_KEY` リポジトリシークレットが未設定。API キーなしで Codex サーバーが起動できないため server info ファイルが作成されず、ポーリングタイムアウトでエラー終了する。

## 実装プラン

手動作業のみ: GitHub リポジトリの Settings → Secrets and variables → Actions で `OPENAI_API_KEY` を設定する。コード修正は不要。

## 結果

コード修正なし。ユーザーが `OPENAI_API_KEY` シークレットを手動設定することで解決する。

## 完了時刻
2026-04-13T14:17:27+09:00
