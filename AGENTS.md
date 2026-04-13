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

## Review guidelines

- レビューは必ず日本語で行うこと
- コード品質、セキュリティ、パフォーマンス、潜在的バグに注目する
- Rust のイディオムとベストプラクティスに従っているか確認する
- `unsafe` コードの使用がある場合は特に注意してレビューする
- Cloudflare Workers 固有の制約（CPU 時間制限、メモリ制限等）を考慮する
- 些末な指摘（typo、フォーマット等）は P1 未満として扱い、重大な問題を優先する
