.POSIX:

all: clippy fmt test

bench:
	@cargo bench --workspace --verbose

check-fmt:
	@cargo fmt --all --check

clippy:
	@cargo clippy --all-features --all-targets --tests --workspace -- -D warnings

fmt:
	@cargo fmt --all

test:
	@cargo test --workspace

.PHONY: all bench check-fmt clippy fmt test
