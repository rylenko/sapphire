.POSIX:

all: clippy fmt test

check-fmt:
	@cargo fmt --all --check

clippy:
	# Better use this in future:
	# @cargo clippy --all-features --tests --workspace -- -D warnings
	@cargo clippy --all-features --benches --tests --workspace

fmt:
	@cargo fmt --all

test:
	@cargo test --workspace

.PHONY: all check-fmt clippy test
