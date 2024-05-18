.POSIX:

ROOT_DIR = $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
CRATES_DIR = $(ROOT_DIR)crates/

all: clippy fmt

bench:
	@cargo bench --workspace --verbose

check-fmt:
	@cargo fmt --all --check

clean:
	@rm -rf target/

clippy:
	@cargo clippy --all-features --all-targets --tests --workspace -- -D warnings

collect-todos:
	@grep -rn "TODO" $(CRATES_DIR)

fmt:
	@cargo fmt --all

test:
	@cargo test --workspace

.PHONY: all bench check-fmt collect-todos clippy fmt test
