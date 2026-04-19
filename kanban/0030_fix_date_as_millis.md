# Date::now().as_millis() ビルド不能レビュー対応

## 目的
現状 cargo check が通っているためビルド不能ではないと思われるが、`worker::Date::now().as_millis()` が存在せずビルドできないというレビューコメントが来た。ビルドを通したい。

## 要望
worker::Date::now().as_millis()が存在せずビルド不能というレビューコメントが来た修正して欲しい

## レビューコメント
**P1 🟠**: Date::now().as_millis() が存在せずビルド不能

`worker::Date::now()` はミリ秒の `f64` を返す API であり `as_millis()` メソッドは存在しないためコンパイルが通りません。結果として署名検証が利用できず、ワーカーがビルド不能になります。既存のレビュー指摘が未解消です。

_信頼度: 0.74_

## プラン

- コード変更なし
- `worker::Date::now().as_millis()` は worker 0.8.0 に存在する正しい API (`date.rs:58, 65` で確認)
- Codex レビュー（信頼度 0.74）は誤りと確定
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` で検証

## 完了サマリー

- 完了: 2026-04-19T18:13:18+09:00
- **コード変更なし**
- `worker` crate 0.8.0 のソース (`date.rs`) を直接確認し、`Date::now() -> Date` と `as_millis() -> u64` の両方が存在することを確認
- `cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown` が成功（CI と同じコマンド）
- Codex レビューコメント（信頼度 0.74）は事実と異なるため、コードはそのまま維持