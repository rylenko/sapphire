.POSIX:

all: clippy fmt test generate-readme

bench:
	@cargo bench --workspace --verbose

check-fmt:
	@cargo fmt --all --check

clippy:
	@cargo clippy --all-features --all-targets --tests --workspace -- -D warnings

fmt:
	@cargo fmt --all

generate-readme:
	@./generate-readme

test:
	@cargo test --workspace

.PHONY: all bench check-fmt clippy fmt generate-readme test
