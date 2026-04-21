# 0017 renovateをセットアップする

## 開始時刻
2026-04-13T15:26:31+09:00

## タスク概要
Github Actionsにおいてワークフローで使用しているActionのバージョン指定の方法をコミットSHAとするためにRenovateを使用する。
またサプライチェーン攻撃にも対応した設定をして欲しい。
他に設定すべきextendsがあったら追加して欲しい。

## 調査結果

### 現状の `renovate.json`
`config:recommended` のみ設定済み。

### GitHub Actions で使用中のアクション
| ワークフロー | アクション | バージョン指定 |
|---|---|---|
| semgrep.yml | actions/checkout | @v4 |
| build.yml | actions/checkout, dtolnay/rust-toolchain, Swatinem/rust-cache | @v4, @stable, @v2 |
| codex-code-review.yml | actions/checkout, openai/codex-action, actions/github-script | @v5, @v1, @v7 |
| deploy.yml | actions/checkout, dtolnay/rust-toolchain, Swatinem/rust-cache, cloudflare/wrangler-action | @v4, @stable, @v2, @v3 |
| lint.yml | actions/checkout, dtolnay/rust-toolchain, Swatinem/rust-cache | @v4, @stable, @v2 |

### Docker コンテナイメージ
| ワークフロー | イメージ | 指定 |
|---|---|---|
| semgrep.yml | semgrep/semgrep | タグなし（latest） |

すべてのアクションがタグ指定のみで、コミット SHA によるピン留めが未実施。
`semgrep.yml` では Docker コンテナイメージもダイジストなしで使用中。

## 実装プラン
- `config:recommended` を維持（Cargo.toml の依存管理の基本設定）
- `helpers:pinGitHubActionDigestsToSemver` を追加（GitHub Actions をコミット SHA にピン留め）
- `docker:pinDigests` を追加（`semgrep/semgrep` イメージのダイジェストピン留め）
- `minimumReleaseAge: "7 days"` でリリース直後の悪意あるパッケージリスクを軽減
- pin/pinDigest の automerge 設定
- GitHub Actions patch/minor の週次 automerge 設定

## 編集したファイル
- `renovate.json` — extends に `helpers:pinGitHubActionDigestsToSemver` と `docker:pinDigests` を追加、セキュリティ設定を追記

## 実行したコマンド
（なし）

## 判断・意思決定
- サンプルの extends から `config:recommended` が抜けていたが、Cargo 依存管理のデフォルト設定として必要なため維持した
- `docker:pinDigests` はサンプルにないが、`semgrep.yml` が Docker コンテナイメージをタグなしで使用しているため追加が必要と判断

## エラー・問題
（なし）

## 完了日時
2026-04-13T15:26:31+09:00
