PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
DOCKER := docker-compose exec keosd
CLEOS := docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
EOSIOCPP := $(DOCKER) eosiocpp
DIST := $(PWD)/dist

docker:
	docker build \
		--tag sagansoftware/eos:v1.1.3 \
		--build-arg eos_branch=v1.1.3 \
		--memory 12G \
		./docker
.PHONY: docker

clean:
	docker kill $(docker ps -q)
.PHONY: clean

full-clean: clean
	docker rm $(docker ps -a -q)
	docker rmi $(docker images -q)
	docker system prune
.PHONY: full-clean

init-chain: build
	$(CLEOS) wallet create
	$(CLEOS) wallet import --private-key $(PRIVKEY)
	$(CLEOS) create account eosio hello $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio hellorust $(PUBKEY) $(PUBKEY)
.PHONY: init-chain

$(DIST):
	mkdir -p $(DIST)

start:
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	docker-compose up
.PHONY: start


build_docker:
	cd /build && \
		cmake /contract && \
		make
.PHONY: build_docker

build:
	mkdir -p ./build
	docker run \
		--interactive \
		--tty \
		--rm \
		--workdir /contract \
		--volume $(PWD):/contract:ro \
		--volume $(PWD)/build:/build \
		--entrypoint make \
		sagansoftware/eos:v1.1.3 \
		build_docker
.PHONY: build

deploy: build
	$(CLEOS) set abi hello /build/hello.abi
	$(CLEOS) set code hello /build/hello.wasm

say_hi:
	$(CLEOS) push action hello hi '["abcd", "1000.00 TGFT"]' -p 'hello@active'

optimize-reference:
	wasm-gc ./build/hello.wasm ./build/hello_gc.wasm
	wasm-opt --output ./build/hello_gc_opt.wasm -Oz ./build/hello_gc.wasm
	wasm2wat ./build/hello_gc_opt.wasm -o ./build/hello_gc_opt.wat --generate-names
	wat2wasm ./build/hello_gc_opt.wat -o ./build/hello_gc_opt_wat.wasm
	wasm-gc ./build/tic_tac_toe.wasm ./build/tic_tac_toe_gc.wasm
	wasm-opt --output ./build/tic_tac_toe_gc_opt.wasm -Oz ./build/tic_tac_toe_gc.wasm
	wasm2wat ./build/tic_tac_toe_gc_opt.wasm -o ./build/tic_tac_toe_gc_opt.wat --generate-names
	wat2wasm ./build/tic_tac_toe_gc_opt.wat -o ./build/tic_tac_toe_gc_opt_wat.wasm
	wasm-gc ./build/eosio.token.wasm ./build/eosio.token_gc.wasm
	wasm-opt --output ./build/eosio.token_gc_opt.wasm -Oz ./build/eosio.token_gc.wasm
	wasm2wat ./build/eosio.token_gc_opt.wasm -o ./build/eosio.token_gc_opt.wat --generate-names
	wat2wasm ./build/eosio.token_gc_opt.wat -o ./build/eosio.token_gc_opt_wat.wasm

build-rust:
	cargo build \
		--target=wasm32-unknown-unknown \
		--release \
		--verbose
	mkdir -p ./build
	rm -f ./build/contract_rust.wasm
	mv -f ../target/wasm32-unknown-unknown/release/contract_rust.wasm ./build
.PHONY: build-rust

deploy-rust: optimize-rust
	$(CLEOS) set abi hellorust /build/hello.abi
	$(CLEOS) set code hellorust /build/contract_rust_gc_opt.wasm

test-rust:
	$(CLEOS) push action hellorust hi '["abcd"]' -p 'hellorust@active'

optimize-rust: build-rust
	wasm-gc ./build/contract_rust.wasm ./build/contract_rust_gc.wasm
	wasm-opt --output ./build/contract_rust_gc_opt.wasm -Oz ./build/contract_rust_gc.wasm
	wasm2wat ./build/contract_rust_gc_opt.wasm -o ./build/contract_rust_gc_opt.wat --generate-names
	wat2wasm ./build/contract_rust_gc_opt.wat -o ./build/contract_rust_gc_opt_wat.wasm
.PHONY: optimize

deploy-rust-from-wat:
	$(CLEOS) set abi hellorust /build/hello.abi
	$(CLEOS) set code hellorust /build/contract_rust_gc_opt_wat.wasm
