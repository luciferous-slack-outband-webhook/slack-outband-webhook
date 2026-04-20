SHELL = /usr/bin/env bash -xeuo pipefail

cargo-check:
	cargo check -p shared -p cli
	cargo check -p slack-outband-webhook-worker --target wasm32-unknown-unknown

cargo-test:
	cargo test -p slack-outband-webhook-worker

.PHONY: \
	cargo-check \
	cargo-test
