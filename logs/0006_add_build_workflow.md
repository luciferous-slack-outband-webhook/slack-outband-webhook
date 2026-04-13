# 0006 ビルドとリントのワークフローを追加する

## 開始日時

2026-04-12T12:18:20+09:00

## 作業ログ

### ステップ1: プランニング

- kanban ファイル読み込み・コードベース調査実施
- 既存ワークフロー（semgrep.yml, deploy.yml, claude.yml, claude-code-review.yml）を確認
- `worker` クレートが wasm32-unknown-unknown ターゲット必須であることを確認
- cargo コマンドをネイティブ（shared, cli）と wasm（worker）に分割する方針を決定
- `cargo check`（`cargo build` の代わり）と `cargo clippy/fmt` の組み合わせで実装する方針を承認

### ステップ2: ワークフローファイル作成

- `.github/workflows/build.yml` を新規作成
  - トリガー: `pull_request` → `master`
  - `cargo check -p shared -p cli`（ネイティブ）
  - `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown`（wasm）
- `.github/workflows/lint.yml` を新規作成
  - トリガー: `pull_request` → `master`
  - `cargo fmt --all --check`
  - `cargo clippy -p shared -p cli -- -D warnings`（ネイティブ）
  - `cargo clippy -p slack-outband-webhook-worker --target wasm32-unknown-unknown -- -D warnings`（wasm）
- 両ファイルとも `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2` を使用

## 完了日時

2026-04-12T12:18:38+09:00
