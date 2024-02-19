.POSIX:

all: clippy check-fmt test

check-fmt:
	@cargo fmt --all --check

clippy:
	@cargo clippy --all --all-features --tests -- -D warnings

test:
	@cargo test --all

.PHONY: all check-fmt clippy test
