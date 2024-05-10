.PHONY: fmt clippy tags-vi test

all: build

build:
	cargo build

clippy:
	cargo clippy -- -Dwarnings

fmt:
	cargo fmt -p camgi -- --check -l

html-designer: build
	./target/debug/camgi demo
	cargo run --example html-designer

tags-vi:
	rusty-tags vi

test:
	cargo test
