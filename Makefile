.PHONY: build run test clean fmt fmt-check clippy taplo taplo-check deny-check pre-release

build:
	bash pwn.sh
	cargo build --all

release:
	bash pwn.sh
	cargo build --release

run:
	bash pwn.sh
	cargo run

test:
	bash pwn.sh
	cargo test

clean:
	cargo clean

fmt:
	bash pwn.sh
	cargo fmt

fmt-check:
	bash pwn.sh
	cargo fmt --all --check

clippy:
	bash pwn.sh
	cargo clippy --all --all-features -- -D warnings

taplo:
	bash pwn.sh
	taplo format

taplo-check:
	bash pwn.sh
	taplo format --check

deny-check:
	bash pwn.sh
	cargo deny --all-features check

.PHONY: pre-release
pre-release:
	make fmt
	make clippy
	make test
	make taplo-check
	make deny-check
