.POSIX:

ROOT_DIR = $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
DESKTOP_CRATE = desktop-iced

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
	@grep -rn \
		--exclude=Makefile \
		--exclude-dir=.git \
		--exclude-dir=target \
		"TODO" $(ROOT_DIR)

fmt:
	@cargo fmt --all

run:
	@cargo run -p $(DESKTOP_CRATE)

test:
	@cargo test --workspace

.PHONY: all bench check-fmt collect-todos clippy fmt test
