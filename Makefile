NPM := `npm bin`
PWD := `pwd`
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
DOCKER := docker-compose exec nodeosd
CLEOS := $(DOCKER) cleos --url http://nodeosd:8888 --wallet-url http://localhost:8888
EOSIOCPP := $(DOCKER) eosiocpp
DIST_WEBSITE := dist/website
DIST_CONTRACT := dist/contract

clean: clean-website
.PHONY: clean

build: clean website
.PHONY: build

install: node_modules
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	rustup component add rustfmt-preview --toolchain nightly
	cargo install --force cargo-watch
	cargo install --force cargo-web
.PHONY: install

node_modules:
	yarn install

cargo:
	cargo web deploy \
		--target=wasm32-unknown-unknown \
		--release \
		--verbose
.PHONY: cargo

clean-cargo:
	rm -Rf target/deploy
.PHONY: clean-cargo

full-clean-cargo:
	rm -Rf target
.PHONY: full-clean-cargo

watch-cargo:
	cargo watch \
		-i '*.css' \
		-i '*.html' \
		-i '*.js*' \
		-i '*.sh' \
		-i 'node_modules' \
		-x "web deploy --target=wasm32-unknown-unknown --release --verbose"
.PHONY: watch-cargo

$(DIST_WEBSITE):
	mkdir -p $(DIST_WEBSITE)

webpack: $(DIST_WEBSITE)
	NODE_ENV=production $(NPM)/webpack
.PHONY: webpack

clean-webpack:
	rm -Rf $(DIST_WEBSITE)/index.js
	rm -Rf $(DIST_WEBSITE)/index.wasm
	rm -Rf $(DIST_WEBSITE)/index.html
.PHONY: clean-webpack

watch-webpack: $(DIST_WEBSITE)
	$(NPM)/webpack --watch
.PHONY: watch-webpack

css: $(DIST_WEBSITE)
	$(NPM)/postcss \
        static/index.css \
        --output $(DIST_WEBSITE)/index.css
.PHONY: css

clean-css:
	rm -Rf $(DIST_WEBSITE)/index.css
.PHONY: clean-css

watch-css: $(DIST_WEBSITE)
	$(NPM)/postcss \
        static/index.css \
        --output $(DIST_WEBSITE)/index.css \
		--watch
.PHONY: watch-css

website:
	$(MAKE) cargo
	$(MAKE) webpack
	$(MAKE) css
.PHONY: website

clean-website: clean-cargo clean-webpack clean-css
.PHONY: clean-website

start-server:
	$(NPM)/browser-sync start \
		--port 1337 \
		--https \
		--single \
		--server $(DIST_WEBSITE) \
		--no-open \
		--no-ui \
		--watch

start-website:
	$(NPM)/concurrently \
		--raw \
		--kill-others \
		"$(MAKE) watch-cargo" \
		"$(MAKE) watch-webpack" \
		"$(MAKE) watch-css" \
		"$(MAKE) start-server"
.PHONY: start-website

test:
	echo TODO
.PHONY: test

docker:
	docker build \
		--tag sagansoftware/eos:v1.1.3 \
		--build-arg eos_branch=v1.1.3 \
		--memory 12G \
		./docker
.PHONY: docker

clean-docker:
	docker kill $(docker ps -q)
.PHONY: clean-docker

full-clean-docker: clean-docker
	docker rm $(docker ps -a -q)
	docker rmi $(docker images -q)
	docker system prune
.PHONY: full-clean-docker

chain-bios:
	$(CLEOS) wallet create
	$(CLEOS) wallet import --private-key $(PRIVKEY)
	$(CLEOS) create account eosio eosio.token $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.msig $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.ram $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.ramfee $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.stake $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.bpay $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.vpay $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.saving $(PUBKEY) $(PUBKEY)
	$(CLEOS) create account eosio eosio.names $(PUBKEY) $(PUBKEY)
	$(CLEOS) set contract eosio.token /eos/build/contracts/eosio.token
	$(CLEOS) set contract eosio.msig /eos/build/contracts/eosio.msig
	$(CLEOS) push action eosio.token create '[ "eosio", "10000000000.0000 SYS" ]' -p eosio.token
	$(CLEOS) push action eosio.token issue '[ "eosio", "10000000000.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) set contract eosio /eos/build/contracts/eosio.system
.PHONY: chain-bios

chain-accounts:
	$(CLEOS) system newaccount eosio --transfer eosstrawpoll $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 2000
	$(CLEOS) system newaccount eosio --transfer alice $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer bob $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer carol $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer williamcurry $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer saganonroids $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer g4ydegenesis $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
.PHONY: chain-accounts

$(DIST_CONTRACT):
	mkdir -p $(DIST_CONTRACT)

contract: $(DIST_CONTRACT)
	# $(EOSIOCPP) \
	# 	--outname /contracts/eosstrawpoll/eosstrawpoll.wast \
	# 	/contracts/eosstrawpoll/eosstrawpoll.cpp
	docker run \
		--interactive \
		--tty \
		--rm \
		--entrypoint eosio-cpp \
		--volume $(PWD)/contract:/contract \
		sagansoftware/eos:v1.1.3 \
		-I=/root/opt/boost/include \
		-o=/contract/test.wasm \
		-O=3 \
		-lto-opt=O3 \
		-fmerge-all-constants \
		-fstrict-return \
		/contract/eosstrawpoll.cpp
.PHONY: contract

test-contract: $(DIST_CONTRACT)
	$(DOCKER) /build_contracts.sh
.PHONY: test-contract