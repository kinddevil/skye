NO_COLOR=\033[0m
OK_COLOR=\033[32;01m

LEVEL_LIST = yichen
FOUND_LEVEL = no


.PHONY: all test fmt build cov clippy release clean

all: clean build

dev:
	RUST_BACKTRACE=1 cargo run --profile dev -Z unstable-options

prod:
	cargo run --profile release -Z unstable-options

test:
	cargo test

fmt:
	cargo +nightly fmt 

build:
	cargo build

cov:
	cargo build && cargo test \
	 && export CARGO_INCREMENTAL=0 && export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
	 && grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/ \
	 && open ./target/debug/coverage/index.html

clippy:
	cargo clippy

release:
	cargo release $$(git log -1 | grep -oE 'release|patch|minor|major|alpha|beta|rc' | head -1) --skip-publish

docker:
	 docker build -t skye:latest . && docker images -qf dangling=true | xargs docker rmi

clean:
	cargo clean

doc:
	cargo doc --open

bi:
	cd data && jupyter lab
