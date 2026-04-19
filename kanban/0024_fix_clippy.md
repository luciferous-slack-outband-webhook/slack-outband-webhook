# clippyの指摘を修正する
## 要望
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

warning: `slack-outband-webhook-worker` (lib) generated 1 warning (run `cargo clippy --fix --lib -p slack-outband-webhook-worker` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```
clippyで上記warningが出た。
これを修正して欲しい。

## 目的
PRのCIとしてclippyを確認している。
エラーになるので修正して欲しい。

## プラン
`worker/src/lib.rs` L64-68 の `if/else` ブロック（5行）を `now_seconds.abs_diff(ts)` の1行に置換する。

## 完了サマリー
完了日時: 2026-04-19T15:43:42+09:00

`worker/src/lib.rs:64-68` の手動絶対差パターン (`if/else`) を `now_seconds.abs_diff(ts)` に1行置換した。
`cargo clippy` で warning なし、`cargo build` 成功を確認。