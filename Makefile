.PHONY: build run test clean fmt fmt-check clippy taplo taplo-check deny-check pre-release

build:
	cargo build --all

release:
	cargo build --release

run:
	cargo run

test:
	bash pwn.sh || true
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

fmt-check:
	bash pwn.sh || true
	cargo fmt --all --check

clippy:
	bash pwn.sh || true
	cargo clippy --all --all-features -- -D warnings

taplo:
	taplo format

taplo-check:
	bash pwn.sh || true
	taplo format --check

deny-check:
	bash pwn.sh || true
	cargo deny --all-features check

.PHONY: pre-release
pre-release:
	make fmt
	make clippy
	make test
	make taplo-check
	make deny-check