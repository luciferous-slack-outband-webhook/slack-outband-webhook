# AGENTS.md

This file provides guidance to OpenAI Codex when working with code in this repository.

## Overview

Slack App の Event Subscription を利用して、メッセージの内容を外部に Webhook するシステム。

- **Cloudflare Workers** が Slack Event Subscription の受け口となり、受信したイベントを外部 Webhook に転送する
- **チャンネル → Webhook 先 URL** のマッピングは **Cloudflare KV** に保持する
- KV へのデータ登録・管理は **CLI** で行う

## Commands

```bash
# ビルド
cargo build
cargo build --release
cargo build -p cli      # 個別パッケージ指定
cargo build -p worker
cargo build -p shared

# チェック・リント
cargo check
cargo clippy
cargo fmt

# テスト
cargo test              # 全ワークスペース
cargo test -p shared    # 個別パッケージ指定
```

## Architecture

Rust Cargo ワークスペース構成。

```
├── cli/       # Cloudflare KV へのデータ登録・管理用 CLI
├── worker/    # Cloudflare Workers 上で動作する Slack Event Subscription の受け口。受信イベントを外部に Webhook する
└── shared/    # cli と worker の共通型・ユーティリティ
```

- `cli` と `worker` はそれぞれ独立したバイナリ。共通コードは `shared` に置く。
- Rust edition 2024 を使用。

## CI

| ワークフロー | 実行タイミング | 内容 |
|---|---|---|
| `build.yml` | PR（master 向け） | `cargo check`（native: `shared`, `cli` / wasm32: `worker`） |
| `lint.yml` | PR | clippy, fmt など |
| `semgrep.yml` | push/PR/日次 | Semgrep による静的解析 |
| `codex-code-review.yml` | PR（non-draft） | Codex による PR レビュー |

### Codex Code Review について

`codex-code-review.yml` では、幻覚によるビルドエラー誤指摘を抑制するため、Codex 実行前に `cargo check` を実行してその結果をプロンプトに注入している。

**ビルドターゲットを追加・変更する場合は `build.yml` と `codex-code-review.yml` の `cargo check` コマンドを必ず同期して更新すること。**

現在の `cargo check` コマンド（両ワークフロー共通）：
- `cargo check -p shared -p cli`（native）
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown`（wasm32）

例：CLI の macOS/Windows 向けクロスコンパイルを追加する場合、両ワークフローに対応する target のチェックステップを追加する。

## Kanban ワークフロー

タスク管理に kanban 方式を採用している。詳細は `.claude/skills/kanban/references/kanban-workflow.md` を参照。

- `kanban/` にタスクファイル（`{xxxx}_{title}.md`）を配置する
- `logs/` に同名のログファイルが自動生成される（git 管理対象）
- `kanban/` と `logs/` は開発者向けの記録であり、Codex Code Review の対象外としている（`.github/workflows/codex-code-review.yml` で作業ツリーから削除している）
- **タスク開始時は `/kanban` スキルを使用すること**
- `/kanban` はまずプランモードで計画を立て、承認後に実装に移る
- **タスク作業中は、各ステップ完了時に必ずログファイルを更新すること**
- kanban ファイルへの追記時・ログへの記録時は JST タイムゾーンの ISO 8601 形式で日時を記載する

## Review guidelines

- レビューは必ず日本語で行うこと
- コード品質、セキュリティ、パフォーマンス、潜在的バグに注目する
- Rust のイディオムとベストプラクティスに従っているか確認する
- `unsafe` コードの使用がある場合は特に注意してレビューする
- Cloudflare Workers 固有の制約（CPU 時間制限、メモリ制限等）を考慮する
- 些末な指摘（typo、フォーマット等）は P1 未満として扱い、重大な問題を優先する
