.PHONY: fmt clippy tags-vi test

all: build

build:
	cargo build

fmt:
	cargo fmt -p camgi --

clippy:
	cargo clippy -- -Dwarnings

tags-vi:
	rusty-tags vi

test:
	cargo test
