# 0024 clippyの指摘を修正する

## ヘッダー
- 開始時刻: 2026-04-19T15:42:48+09:00
- 完了時刻: 2026-04-19T15:43:42+09:00

## タスク概要
clippyで以下のwarningが出た。修正して欲しい。

```
warning: manual absolute difference pattern without using `abs_diff`
  --> worker/src/lib.rs:64:16
   |
64 |       let diff = if now_seconds > ts {
   |  ________________^
65 | |         now_seconds - ts
66 | |     } else {
67 | |         ts - now_seconds
68 | |     };
   | |_____^ help: replace with `abs_diff`: `now_seconds.abs_diff(ts)`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#manual_abs_diff
   = note: `#[warn(clippy::manual_abs_diff)]` on by default
```

PRのCIとしてclippyを確認している。エラーになるので修正して欲しい。

## 調査結果

### worker/src/lib.rs
- L63-L68: `now_seconds`（`u64`）と `ts`（`u64`）の差を `if/else` で手動計算している
- `TIMESTAMP_TOLERANCE_SECONDS`（5分=300秒）と比較し、リプレイ攻撃を防ぐ
- 両変数が `u64` であるため `u64::abs_diff` が直接使用可能（Rust 1.60 stable）
- ロジックは等価: `|now_seconds - ts|` = `now_seconds.abs_diff(ts)`

## 実装プラン
`worker/src/lib.rs` L64-68 の `if/else` ブロック (5行) を `now_seconds.abs_diff(ts)` の 1 行に置換するのみ。

## プランニング経緯
初回提案がそのまま承認された。

## 会話内容
- ユーザーが `/kanban` を実行
- タスク 0024 を選択 (番号最大の未完了タスク)
- clippy warning の内容確認 → `abs_diff` への置換プランを提示
- ユーザー承認

## 編集したファイル
- `worker/src/lib.rs`: L64-68 を `let diff = now_seconds.abs_diff(ts);` に置換

## 実行したコマンド
- `cargo clippy`
- `cargo build`

## 判断・意思決定
- 特になし。clippy の提案通り。

## エラー・問題
- 特になし。

## 完了日時
2026-04-19T15:43:42+09:00
