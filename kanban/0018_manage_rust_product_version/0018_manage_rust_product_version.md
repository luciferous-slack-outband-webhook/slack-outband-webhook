# RustのProductのバージョンを管理する
## 要望
Cargo Workspaceで管理しているRustのProductのバージョンをルートのCargo.tomlで管理できるようにしたい。
最初のバージョンは `0.0.1` とする。

## プラン

1. ルート `Cargo.toml` に `[workspace.package]` を追加（`version = "0.0.1"`, `edition = "2024"`）
2. `cli/`, `worker/`, `shared/` の `Cargo.toml` で `version.workspace = true` / `edition.workspace = true` に変更

## 完了サマリー

完了日時: 2026-04-13T18:25:21+09:00

`[workspace.package]` を使ってバージョン (`0.0.1`) と edition を一元管理するよう変更した。
`cargo check` / `cargo clippy` ともにエラー・警告なしで完了。