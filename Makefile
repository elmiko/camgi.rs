.PHONY: fmt clippy tags-vi test

fmt:
	cargo fmt -p camgi -- --check -l

clippy:
	cargo clippy -- -Dwarnings

tags-vi:
	rusty-tags vi

test:
	cargo test
