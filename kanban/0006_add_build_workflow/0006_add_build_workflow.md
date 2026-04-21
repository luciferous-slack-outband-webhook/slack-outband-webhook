# ビルドとリントのワークフローを追加する
## 要望

masterブランチに対するPRで起動するワークフローを二つ追加する。

1. workspace全体でビルドが通るかどうか
2. workspace全体でリントを行う

## プラン

- `.github/workflows/build.yml`: PR→master でトリガー、`cargo check` でネイティブ（shared/cli）と wasm32（worker）に分割実行
- `.github/workflows/lint.yml`: PR→master でトリガー、`cargo fmt --check` + `cargo clippy -D warnings` をネイティブ/wasm32 分割実行
- ツールチェイン: `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`

## 完了サマリー

**完了日時**: 2026-04-12T12:18:38+09:00

- `.github/workflows/build.yml` を新規作成
- `.github/workflows/lint.yml` を新規作成
- `worker` クレートの wasm32 ターゲット制約に対応するため、全 cargo コマンドをネイティブ（shared/cli）と wasm32（worker）に分割した
- `cargo build` の代わりに `cargo check` を使用（高速かつ同等のコンパイルエラー検出）
