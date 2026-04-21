# 0021_add_makefile

## ヘッダー

- 開始時刻: 2026-04-13T21:07:58+09:00
- 完了時刻: 2026-04-13T21:08:48+09:00

## タスク概要

Makefileを作り `cargo check` できるようにする。

## 調査結果

- プロジェクトルートに Makefile は存在しない
- ワークスペース構成: `shared`（lib）、`cli`（binary）、`worker`（cdylib / WASM）
- `worker` クレートは `wasm32-unknown-unknown` ターゲットが必要で、単純な `cargo check` では不十分
- CI（`.github/workflows/build.yml`）では以下の2コマンドで check を実施:
  - `cargo check -p shared -p cli`
  - `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown`

## 実装プラン

- `cargo-check` ターゲットのみを持つ `Makefile` をプロジェクトルートに新規作成
- 上記2つの `cargo check` コマンドを順番に実行する

## 実装フェーズ

### 編集したファイル

- 新規作成: `Makefile`

### 実行したコマンド

- `make cargo-check` で動作確認

### 判断・意思決定

- ユーザー指示により `cargo-check` ターゲットのみ（他のターゲットは含めない）

### エラー・問題

- なし
