.PHONY: all
all: fmt check

.PHONY: fmt
fmt:
	@cargo +nightly fmt --all

.PHONY: check
check:
	@cargo +nightly check

.PHONY: build
build:
	@cargo +nightly build

.PHONY: test
test:
	@cargo +nightly test

.PHONY: itest
itest:
	@RUST_LOG=off cargo +nightly test -- --ignored

.PHONY: lint
lint:
	@cargo +nightly clippy

.PHONY: clean
clean:
	@cargo +nightly clean

.PHONY: run
run:
	@cargo +nightly run

.PHONY: release
release:
	@cargo +nightly build --release
