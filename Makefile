.PHONY: fmt clippy test

fmt:
	cargo fmt -p camgi -- --check -l

clippy:
	cargo clippy -- -Dwarnings

test:
	cargo test
