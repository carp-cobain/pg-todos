.PHONY: all
all: fmt build test lint

.PHONY: fmt
fmt:
	@cargo fmt --all

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: lint
lint:
	@cargo clippy

.PHONY: clean
clean:
	@cargo clean

.PHONY: release
release:
	@cargo build --release

.PHONY: migrate
migrate:
	@diesel migration run
