# Codex Reviewのプロンプトを更新する
## 要望
Codex Reviewのプロンプトにおいて、新たに導入された問題のみ指摘するようになっている。
それ以外の問題も指摘するようにして欲しい。

## 理由
細かい単位でカンバンを作成しているため問題を後のPRで対応するというのを多用している。
あとで問題が見えなくなると問題なので指摘はして欲しい。

## プラン

1. `.github/codex/codex-prompt.md`: 「新たに導入された問題のみ」→「差分に含まれる全ての問題（既存も含む）」
2. `.github/workflows/codex-code-review.yml`: 過去コメント指示を「findings として再指摘」に変更

## 完了サマリー

- 完了日時: 2026-04-19T15:28:55+09:00
- `.github/codex/codex-prompt.md` のレビュー方針を変更し、差分に表れている既存の問題も指摘対象に含めた
- `.github/workflows/codex-code-review.yml` の過去コメント指示を変更し、未対応の指摘を findings 配列として再報告するよう更新した