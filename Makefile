.POSIX:

ROOT_DIR = $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
GEN_README_PATH = $(ROOT_DIR)readme-gen/run

all: clippy fmt test gen-readme

bench:
	@cargo bench --workspace --verbose

check-fmt:
	@cargo fmt --all --check

clippy:
	@cargo clippy --all-features --all-targets --tests --workspace -- -D warnings

fmt:
	@cargo fmt --all

gen-readme:
	@$(GEN_README_PATH)

test:
	@cargo test --workspace

.PHONY: all bench check-fmt clippy fmt gen-readme test
