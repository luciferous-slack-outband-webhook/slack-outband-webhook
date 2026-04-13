# Makefileを追加しcheckできるようにする
## 要望
Makefileを作り `cargo check` できるようにする。

## プラン

- `cargo-check` ターゲットのみを持つ `Makefile` をプロジェクトルートに新規作成
- native（`shared`, `cli`）と wasm32（`worker`）の両方を check する

## 完了サマリー

- 完了日時: 2026-04-13T21:08:48+09:00
- プロジェクトルートに `Makefile` を新規作成
- `make cargo-check` で `cargo check -p shared -p cli` と `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` の両方を実行できるようになった
- 動作確認済み（両チェックともエラーなし）