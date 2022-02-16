.PHONY: fmt clippy test

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -Dwarnings

test:
	cargo test
