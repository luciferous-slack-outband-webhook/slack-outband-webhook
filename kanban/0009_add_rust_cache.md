# デプロイ時もRustのCacheを有効にする
## 要望
```
**3. `wrangler.toml`: ビルドコマンドが毎回 `cargo install worker-build` を実行する**

\`\`\`toml
command = "cargo install worker-build && worker-build --release"
\`\`\`

`cargo install` はソースからコンパイルするため、デプロイのたびに数分かかります。`deploy.yml` には `Swatinem/rust-cache` が設定されていないため、`~/.cargo/bin/worker-build` もキャッシュされません。対策として：

- `deploy.yml` に `Swatinem/rust-cache@v2` を追加し `~/.cargo/bin` もキャッシュ対象に含める
- またはビルドステップを CI 側に切り出して `wrangler deploy --no-bundle` 方式に変更する
```

前者のキャッシュに含める方向で作業してください

## プラン

- `.github/workflows/deploy.yml` に `Swatinem/rust-cache@v2` を追加
- `cache-directories: "~/.cargo/bin"` を指定し `worker-build` バイナリもキャッシュ対象に含める

## 完了サマリー

- 完了日時: 2026-04-12T15:34:20+09:00
- `.github/workflows/deploy.yml` の `dtolnay/rust-toolchain` と `cloudflare/wrangler-action` の間に `Swatinem/rust-cache@v2` ステップを追加した
- `cache-directories: "~/.cargo/bin"` を設定し、`cargo install worker-build` でインストールされる `worker-build` バイナリをキャッシュ対象に含めた
- `build.yml` / `lint.yml` と同じ `Swatinem/rust-cache@v2` を使用し、一貫性を保った