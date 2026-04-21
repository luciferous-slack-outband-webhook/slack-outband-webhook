# renovateをセットアップする
## 要望
Github Actionsにおいてワークフローで使用しているActionのバージョン指定の方法をコミットSHAとするためにRenovateを使用する。

またサプライチェーン攻撃にも対応した設定をして欲しい。
一応サンプルは用意した。

他に設定すべきextendsがあったら追加して欲しい。
(そのときには何故追加するのか説明が欲しい)

## プラン
- `config:recommended` を維持（Cargo.toml の依存管理の基本設定）
- `helpers:pinGitHubActionDigestsToSemver` を追加（GitHub Actions をコミット SHA にピン留め）
- `docker:pinDigests` を追加（`semgrep/semgrep` イメージのダイジェストピン留め）
- `minimumReleaseAge: "7 days"` でリリース直後の悪意あるパッケージリスクを軽減
- pin/pinDigest の automerge 設定
- GitHub Actions patch/minor の週次 automerge 設定

## サンプル
```json
{
  "$schema": "https://docs.renovatebot.com/renovate-schema.json",
  "extends": [
    "helpers:pinGitHubActionDigestsToSemver"
  ],
  "minimumReleaseAge": "7 days",
  "packageRules": [
    {
      "matchUpdateTypes": ["pin", "pinDigest"],
      "groupName": "version pinning",
      "schedule": "at any time",
      "automerge": true
    }
  ],
  "github-actions": {
    "packageRules": [
      {
        "matchDepTypes": ["action"],
        "matchUpdateTypes": ["patch", "minor"],
        "groupName": "GitHub Actions",
        "schedule": "before 4am on monday",
        "automerge": true
      }
    ]
  }
}
```

## 完了サマリー
完了日時: 2026-04-13T15:26:31+09:00

`renovate.json` を更新し、以下の設定を追加した。

- **`helpers:pinGitHubActionDigestsToSemver`**: GitHub Actions のバージョン指定をコミット SHA にピン留め（semver タグはコメントで維持）。タグ書き換えによるサプライチェーン攻撃を防ぐ。
- **`docker:pinDigests`**: Docker コンテナイメージをダイジェスト（SHA256）にピン留め。`semgrep.yml` で `semgrep/semgrep` イメージがタグなしで使用されているため追加が必要と判断。
- **`minimumReleaseAge: "7 days"`**: リリースから7日間は更新を提案しない。悪意あるパッケージが公開直後に検出される猶予を確保するサプライチェーン対策。
- **pin/pinDigest の automerge**: 初回ピン留め PR を自動マージ（バージョン変更なく安全）。
- **GitHub Actions patch/minor の週次 automerge**: 月曜朝4時前に自動マージ。