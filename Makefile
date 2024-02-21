.POSIX:

all: clippy fmt test

check-fmt:
	@cargo fmt --all --check

clippy:
	@cargo clippy --all-features --tests --workspace -- -D warnings

fmt:
	@cargo fmt --all

test:
	@cargo test --workspace

.PHONY: all check-fmt clippy test
