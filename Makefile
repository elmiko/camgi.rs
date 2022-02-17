.PHONY: fmt clippy test

fmt:
	cargo fmt -p camgi --check

clippy:
	cargo clippy -- -Dwarnings

test:
	cargo test
