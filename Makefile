.PHONY: build test lint fmt fmt-check ci codegen publish

build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

ci: fmt-check lint test

codegen:
	cargo run -p iuliia-codegen

publish:
	cargo publish -p iuliia
