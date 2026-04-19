# ログ: 0031 Codex Reviewが誤っている

## 開始日時
2026-04-20T00:28:29+09:00

## タスク概要

Codex Reviewにおいて、誤った指摘が続いている。特に `worker::Date::now().as_millis()` について「ビルド不能」との誤指摘が出続けている（信頼度 0.39 / 0.74）。CIでビルドできることは確認済み（kanban 0030 にて検証）。今後もビルドエラー系の幻覚が出続けると想定されるため、根本的に抑制する仕組みを整える。

## 調査結果

### エラー内容（kanban ファイルより）

```
**P1 🟠**: 時刻取得APIの誤用でビルド不能

`worker` 0.8系の `Date::now()` はミリ秒の `f64` を返すAPIであり `as_millis()` メソッドは存在しません。
ここで `as_millis()` を呼び出すとコンパイルが通らず、署名検証が実行不可能になります。

_信頼度: 0.39_
```

kanban 0030 では信頼度 0.74 で同様の指摘が来ていた。

### kanban 0030 の調査済み事実（完了サマリーより）

- `worker` crate 0.8.0 のソース (`date.rs`) を直接確認し、`Date::now() -> Date` と `as_millis() -> u64` の両方が存在することを確認
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` が成功（CI と同じコマンド）
- Codex の指摘は事実と異なる → Codex が read-only sandbox で外部クレートの API を幻覚で指摘している

### codex-code-review.yml の構成

- `.github/workflows/codex-code-review.yml` が Codex Review を実行
- Codex は `openai/codex-action@v1` を `sandbox: read-only` で実行
- PR merge commit を checkout し、kanban/logs を削除してから Codex に渡す
- 過去のレビューコメントを `past-review-context.md` に収集してプロンプトに注入
- `Build review prompt` ステップで `.github/codex/codex-prompt.md` にコンテキストを追記して実行
- Codex の出力は `codex-output.json`（構造化 JSON）→ `Post inline review comments` ステップで PR に投稿

### codex-prompt.md の現状

```markdown
あなたは別のエンジニアが作成したコード変更のレビュアーです。

## レビュー方針

正確性、パフォーマンス、セキュリティ、保守性、開発者体験に影響する問題に焦点を当ててください。
このPRの差分に含まれるすべての問題を指摘してください。新たに導入された問題だけでなく、差分に表れている既存の問題も指摘対象です。
問題を指摘する際は、短く直接的な説明と、影響を受けるファイルおよび行範囲を記載してください。
重大な問題を優先し、理解を妨げない限りnit（些末な指摘）は避けてください。

すべての指摘を列挙した後、パッチ全体の正確性に関する総合判定（"patch is correct" または "patch is incorrect"）と、
簡潔な根拠および 0〜1 の信頼度スコアを出力してください。

ファイルの引用と行番号は、利用可能なツールを使って**正確に**確認してください。
不正確な場合、コメントは拒否されます。
...
## PR 情報  （ここにPR差分が注入される）
```

「正確に確認してください」とあるが、read-only sandbox 内で外部クレートのソースは参照できないため幻覚が生まれる。

### build.yml の構成

```yaml
- name: Check native crates
  run: cargo check -p shared -p cli

- name: Check worker crate (wasm32)
  run: cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown
```

`dtolnay/rust-toolchain` + `wasm32-unknown-unknown` target + `Swatinem/rust-cache` のセット。

### codex-output-schema.json の構造

- `findings[]`: 指摘リスト（`confidence_score` 0〜1, `priority` 0〜3, `code_location`）
- `overall_correctness`: "patch is correct" or "patch is incorrect"
- `overall_confidence_score`: 0〜1
- `summary`: string

confidence_score フィルタはユーザー判断で「使わない」選択。

## 実装プラン

### ユーザー選択

- 対策粒度: **案2 中程度**（プロンプト強化 + cargo check 結果をプロンプトに注入）
- `confidence_score` フィルタ: **使わない**

### 変更1: `.github/workflows/codex-code-review.yml`

`Strip kanban and logs from workspace` の後に以下を挿入：

1. `Setup Rust toolchain` - `dtolnay/rust-toolchain@29eef336d9b2848a0b548edc03f92a220660cdb8` + `wasm32-unknown-unknown`
2. `Cache dependencies` - `Swatinem/rust-cache@c19371144df3bb44fab255c43d04cbc2ab54d1c4`
3. `Run cargo check (native)` - `id: check-native`, `continue-on-error: true`, `cargo check -p shared -p cli 2>&1 | tee cargo-check-native.log`
4. `Run cargo check (worker wasm)` - `id: check-wasm`, `continue-on-error: true`, `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown 2>&1 | tee cargo-check-wasm.log`

`Build review prompt` ステップに env と出力ブロックを追加：
- `NATIVE_CHECK_OUTCOME: ${{ steps.check-native.outcome }}`
- `WASM_CHECK_OUTCOME: ${{ steps.check-wasm.outcome }}`
- 差分の前に「## ビルド状況 (cargo check)」ブロックを注入
- 両方 success なら「ビルドエラー指摘禁止」メッセージ
- failure があれば該当ログを埋め込む

### 変更2: `CLAUDE.md`

CI セクションに Codex Review の記述を追加し、将来ビルドターゲット追加時に codex-code-review.yml も同期更新が必要なことを明記。

### 変更3: `.github/codex/codex-prompt.md`

末尾に「ビルドエラー系の指摘に関する制約」セクションを追加。

## プランニング経緯

- 最初に提示した3案（軽量・中程度・重量）からユーザーが案2を選択
- confidence_score フィルタはユーザー判断で「使わない」
- ExitPlanMode 承認後にユーザーから「CLAUDE.md にも記載して将来のビルドターゲット追加時に同期更新されるようにしたい（CLI の macOS/Windows ビルドも予定）」という追加指示あり → プランに反映して再承認
- 初回提案がほぼそのまま承認された（CLAUDE.md 追加を除く）

## 会話内容

1. `/kanban 0031` 呼び出し
2. タスクファイル読み込み・調査（workflow, prompt, schema, build.yml, 0030 kanban）
3. advisor に相談（3案を提示してユーザー選択を推奨する助言を得た）
4. EnterPlanMode → AskUserQuestion で2問（粒度、信頼度フィルタ閾値）を提示
5. ユーザー: 案2・フィルタなし を選択
6. プランファイル作成 → ExitPlanMode
7. ユーザーが「CLAUDE.md に Codex Review の記述も追加してほしい、将来ビルドターゲット追加時に同期更新されるようにしたい（macOS/Windows CLI も予定）」とリジェクト
8. プランに CLAUDE.md 変更を追加 → 再承認

## 編集したファイル

- [x] `kanban/0031_update_codex_review.md` - `## プラン` セクション追記
- [x] `.github/workflows/codex-code-review.yml` - Rust toolchain + cargo check ステップ追加、Build review prompt 修正
- [x] `CLAUDE.md` - CI セクション拡張（AGENTS.md も hook で自動同期）
- [x] `.github/codex/codex-prompt.md` - ビルドエラー系制約セクション追加

## 実行したコマンド

- `git diff --stat` で変更ファイル確認
- `git diff AGENTS.md` で AGENTS.md が hook で自動同期されていることを確認
- `git diff .github/workflows/codex-code-review.yml` で変更内容確認

## 判断・意思決定

- AGENTS.md は CLAUDE.md の変更が `sync-agents-md.sh` フックで自動同期される仕組みになっているため、追加の編集は不要と判断した
- cargo check に `pipefail` を設定しないのは、`| tee` との組み合わせで cargo の終了コードが失われるため。`continue-on-error: true` と `steps.*.outcome` で判定する設計が正しい
- ビルド状況ブロックは過去コメントブロックの前に配置した（Codex が PR 情報を読む前に制約を把握させるため）

## エラー・問題

なし

## 完了日時

2026-04-20T00:30:57+09:00
