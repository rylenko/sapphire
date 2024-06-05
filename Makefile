.POSIX:

include config.mk

ROOT_DIR = $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: clippy fmt

bench:
	@cargo bench --workspace --verbose

build:
	@cargo build -p $(BIN_CRATE) --release

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

install:
	@mkdir -p $(INSTALL_PREFIX)/bin
	@cp ./target/release/$(BIN_CRATE) $(INSTALL_PREFIX)/bin/$(INSTALL_BIN_NAME)

run-debug:
	@cargo run -p $(BIN_CRATE)

test:
	@cargo test --workspace

uninstall:
	@rm -rf $(INSTALL_PREFIX)/bin/$(INSTALL_BIN_NAME)

.PHONY: all bench check-fmt collect-todos clippy fmt test
