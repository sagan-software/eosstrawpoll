install:
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	rustup component add rustfmt-preview --toolchain nightly
	cargo install --force cargo-watch
	cargo install --force cargo-web

build:
	cargo web build --target=wasm32-unknown-unknown --release

start:
	cargo web start --target=wasm32-unknown-unknown --release

BUILD_CONTRACTS := docker run -i -t --rm \
	-v `pwd`/build_contracts.sh:/build_contracts.sh \
	-v `pwd`/contract:/contracts/eosstrawpoll \
	--entrypoint /build_contracts.sh \
	sagansoftware/eos:v1.1.0

build-contract:
	$(BUILD_CONTRACTS)
