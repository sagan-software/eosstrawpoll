all: install build test

install:
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	cargo install --force wasm-gc
	cargo install --force bindgen
	cargo install --force wasm-bindgen
	cd crates/website && npm install

build: build-contract website

test:
	cargo test --features test

clean:
	rm -Rf target

DOCKER_COMPOSE := docker-compose -f docker/docker-compose.yml
CLEOS := $(DOCKER_COMPOSE) exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

docker-down:
	$(DOCKER_COMPOSE) down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume

docker: docker-down
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	$(DOCKER_COMPOSE) up


init:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)
	$(CLEOS) create account eosio eosstrawpoll $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio alice $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio bob $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio carol $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.token $(PUBKEY) $(PUBKEY)
	$(CLEOS) set abi eosio.token /mnt/dev/docker/eosio_token.abi.json
	$(CLEOS) set code eosio.token /mnt/dev/docker/eosio_token.wasm
	$(CLEOS) push action eosio.token create '[ "eosio", "10000000000.0000 SYS" ]' -p eosio.token
	$(CLEOS) push action eosio.token issue '[ "eosio", "10000000000.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "eosstrawpoll", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "alice", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "bob", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "carol", "2500.0000 SYS", "memo" ]' -p eosio

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_gc_opt.wasm: %_gc.wasm
	wasm-opt --fuzz-exec --output $*_gc_opt.wasm -Oz $*_gc.wasm

%_gc_opt.wat: %_gc_opt.wasm
	wasm2wat $*_gc_opt.wasm -o $*_gc_opt.wat --generate-names

%_gc_opt_wat.wasm: %_gc_opt.wat
	wat2wasm $*_gc_opt.wat -o $*_gc_opt_wat.wasm

# --------
# CONTRACT
# --------

build-contract: build-contract-wasm target/wasm32-unknown-unknown/release/contract_gc_opt_wat.wasm

build-contract-wasm:
	cargo build -vv --release --target=wasm32-unknown-unknown --package contract

contract: build-contract
	$(CLEOS) set abi eosstrawpoll /mnt/dev/contract/contract.abi.json
	$(CLEOS) set code eosstrawpoll /mnt/dev/release/contract_gc_opt.wasm

.PHONY: contract

createpoll:
	$(CLEOS) push action eosstrawpoll createpoll '["test","alice","This is a test poll",[],1,1,1,true,[],0,0]' -p 'alice@active'

destroypoll:
	$(CLEOS) push action eosstrawpoll destroypoll '["test"]' -p 'alice@active'

getpolls:
	$(CLEOS) get table eosstrawpoll eosstrawpoll polls

getvotes:
	$(CLEOS) get table eosstrawpoll eosstrawpoll votes

getpopular:
	$(CLEOS) get table eosstrawpoll eosstrawpoll popularpolls

getnew:
	$(CLEOS) get table eosstrawpoll eosstrawpoll newpolls

# -------
# WEBSITE
# -------

PWD := $(shell pwd)
WEBSITE_DIR := $(PWD)/crates/website
STATIC_DIR := $(WEBSITE_DIR)/static
NODE_MODULES := $(WEBSITE_DIR)/node_modules
NPM := $(NODE_MODULES)/.bin
DIST := $(PWD)/docs
TARGET_DIR := $(PWD)/target/wasm32-unknown-unknown/release
WASM_BINDGEN_DIR := $(TARGET_DIR)/wasm-bindgen

website: website-wasm css webpack

website-wasm:
	cargo +nightly build \
		--target wasm32-unknown-unknown \
		--release \
		--package website
	mkdir -p $(WASM_BINDGEN_DIR)
	wasm-bindgen \
		$(TARGET_DIR)/website.wasm \
		--out-dir $(WASM_BINDGEN_DIR)
	wasm-gc \
		$(WASM_BINDGEN_DIR)/website_bg.wasm \
		$(WASM_BINDGEN_DIR)/website_bg_gc.wasm
	wasm-opt --fuzz-exec \
		--output $(WASM_BINDGEN_DIR)/website_bg_gc_opt.wasm -Oz \
		$(WASM_BINDGEN_DIR)/website_bg_gc.wasm
	mv \
		$(WASM_BINDGEN_DIR)/website_bg.wasm \
		$(WASM_BINDGEN_DIR)/website_bg_orig.wasm
	cp \
		$(WASM_BINDGEN_DIR)/website_bg_gc_opt.wasm \
		$(WASM_BINDGEN_DIR)/website_bg.wasm
	

start-website-wasm:
	cargo watch -w $(WEBSITE_DIR)/src -s "make website-wasm"

css:
	NODE_PATH=$(NODE_MODULES) $(NPM)/postcss $(STATIC_DIR)/index.css --output $(DIST)/index.css

start-css:
	NODE_PATH=$(NODE_MODULES) $(NPM)/postcss $(STATIC_DIR)/index.css --output $(DIST)/index.css --watch

webpack:
	$(NPM)/env-cmd $(WEBSITE_DIR)/.env.prod $(NPM)/webpack --config $(WEBSITE_DIR)/webpack.config.js

start-webpack:
	$(NPM)/env-cmd $(WEBSITE_DIR)/.env.dev $(NPM)/webpack --watch --config $(WEBSITE_DIR)/webpack.config.js

start-server:
	$(NPM)/browser-sync start --single --config $(WEBSITE_DIR)/bs-config.js

start-website:
	$(NPM)/concurrently \
		--raw \
		--kill-others \
		"$(MAKE) start-website-wasm" \
		"$(MAKE) start-webpack" \
		"$(MAKE) start-css" \
		"$(MAKE) start-server"

.PHONY: install build test clean docker wallet website gh-pages
.SECONDARY:
