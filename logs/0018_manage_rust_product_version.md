# 0018: RustのProductのバージョンを管理する

開始日時: 2026-04-13T18:23:02+09:00

## タスク概要

Cargo Workspaceで管理しているRustのProductのバージョンをルートのCargo.tomlで管理できるようにしたい。
最初のバージョンは `0.0.1` とする。

## 調査結果

### 調べたファイル

- `Cargo.toml` (ルート): `[workspace]` セクションのみ。members に `shared`, `worker`, `cli` が含まれる。`resolver = "3"` を使用。
- `cli/Cargo.toml`: `version = "0.1.0"`, `edition = "2024"`
- `worker/Cargo.toml`: `version = "0.1.0"`, `edition = "2024"`
- `shared/Cargo.toml`: `version = "0.1.0"`, `edition = "2024"`

### 現状の構造

各メンバーが独自に `version` と `edition` を持っている。Cargo の `[workspace.package]` 機能を使えば、これらをルートで一元管理できる。

## 実装プラン

### Step 1: ルート `Cargo.toml` に `[workspace.package]` を追加

```toml
[workspace.package]
version = "0.0.1"
edition = "2024"
```

### Step 2: 各メンバーの `Cargo.toml` を更新

`version` と `edition` をワークスペース参照に変更：
- `cli/Cargo.toml`
- `worker/Cargo.toml`
- `shared/Cargo.toml`

## 実装フェーズ

完了日時: 2026-04-13T18:25:21+09:00

### 編集したファイル

- `Cargo.toml`: `[workspace.package]` セクションを追加（`version = "0.0.1"`, `edition = "2024"`）
- `cli/Cargo.toml`: `version`/`edition` を `*.workspace = true` に変更
- `worker/Cargo.toml`: `version`/`edition` を `*.workspace = true` に変更
- `shared/Cargo.toml`: `version`/`edition` を `*.workspace = true` に変更

### 実行したコマンド

- `cargo check` → `Finished` (エラーなし)
- `cargo clippy` → 全パッケージ `v0.0.1` として認識、警告なし

### 判断・意思決定

`edition` もワークスペース管理に統一することで、将来の edition アップグレード時も一箇所の変更で済むようにした。

### エラー・問題

なし

