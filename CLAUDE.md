# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

`.github/workflows/semgrep.yml` が Semgrep による静的解析を実行する（push/PR/日次スケジュール）。

## Kanban ワークフロー

タスク管理に kanban 方式を採用している。詳細は `.claude/kanban-workflow.md` を参照。

- `kanban/` にタスクファイル（`{xxxx}_{title}.md`）を配置する
- `logs/` に同名のログファイルが自動生成される（git 管理対象）
- `kanban/` と `logs/` は開発者向けの記録であり、Codex Code Review の対象外としている（`.github/workflows/codex-code-review.yml` で作業ツリーから削除している）
- **タスク開始時は `/kanban` コマンドを使用すること**
- `/kanban` はまずプランモードで計画を立て、承認後に実装に移る
- **タスク作業中は、各ステップ完了時に必ずログファイルを更新すること**
- kanban ファイルへの追記時・ログへの記録時は JST タイムゾーンの ISO 8601 形式で日時を記載する
