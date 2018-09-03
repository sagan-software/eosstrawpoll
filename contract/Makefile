PWD := `pwd`
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
TELOS_PUBKEY := TLOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
DOCKER := docker-compose exec nodeosd
CLEOS := $(DOCKER) cleos --url http://127.0.0.1:8888 --wallet-url http://127.0.0.1:8888
CLEOS_JUNGLE := $(DOCKER) cleos --url https://api.jungle.alohaeos.com:443 --wallet-url http://127.0.0.1:8888
TECLOS := docker-compose exec tkeosd teclos --url http://tnodeosd:8888 --wallet-url http://127.0.0.1:10900
TECLOS_TESTNET := $(DOCKER) cleos --url https://api.jungle.alohaeos.com:443 --wallet-url http://127.0.0.1:8888
EOSIOCPP := $(DOCKER) eosiocpp
DIST := $(PWD)/dist

docker:
	docker build \
		--tag sagansoftware/eos:v1.1.3 \
		--build-arg eos_branch=v1.1.3 \
		--memory 12G \
		./docker
.PHONY: docker

docker-telos:
	docker build \
		--tag sagansoftware/telos:stage2.0 \
		--build-arg telos_branch=stage2.0 \
		--memory 12G \
		--file ./docker/Dockerfile.telos \
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
	$(CLEOS) set contract eosio.token /build/eosio.contracts/eosio.token
	$(CLEOS) set contract eosio.msig /build/eosio.contracts/eosio.msig
	$(CLEOS) push action eosio.token create '[ "eosio", "10000000000.0000 SYS" ]' -p eosio.token
	$(CLEOS) push action eosio.token issue '[ "eosio", "10000000000.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) set contract eosio /build/eosio.contracts/eosio.system
.PHONY: chain-bios

chain-accounts:
	$(CLEOS) system newaccount eosio --transfer eosstrawpoll $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 2000
	$(CLEOS) system newaccount eosio --transfer alice $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer bob $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer carol $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer williamcurry $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer saganonroids $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer g4ydegenesis $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer haley $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer clara $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer christine $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer eunice $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer larry $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer janie $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer rod $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer becky $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer helen $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) system newaccount eosio --transfer chuck $(PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(CLEOS) push action eosio.token transfer '[ "eosio", "eosstrawpoll", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "alice", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "bob", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "carol", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "williamcurry", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "saganonroids", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "g4ydegenesis", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "haley", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "clara", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "christine", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "eunice", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "larry", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "janie", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "rod", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "becky", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "helen", "2500.0000 SYS", "memo" ]' -p eosio
	$(CLEOS) push action eosio.token transfer '[ "eosio", "chuck", "2500.0000 SYS", "memo" ]' -p eosio
.PHONY: chain-accounts

init-chain: build chain-bios chain-accounts deploy
.PHONY: init-chain

chain-bios-telos:
	$(TECLOS) wallet create --to-console
	$(TECLOS) wallet import --private-key $(PRIVKEY)
	$(TECLOS) create account eosio eosio.token $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.msig $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.ram $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.ramfee $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.stake $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.bpay $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.vpay $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.saving $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) create account eosio eosio.names $(TELOS_PUBKEY) $(TELOS_PUBKEY)
	$(TECLOS) set contract eosio.token /build/eosio.contracts/eosio.token
	$(TECLOS) set contract eosio.msig /build/eosio.contracts/eosio.msig
	$(TECLOS) push action eosio.token create '[ "eosio", "10000000000.0000 SYS" ]' -p eosio.token
	$(TECLOS) push action eosio.token issue '[ "eosio", "10000000000.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) set contract eosio /build/eosio.contracts/eosio.system
.PHONY: chain-bios

chain-accounts-telos:
	$(TECLOS) system newaccount eosio --transfer eosstrawpoll $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 2000
	$(TECLOS) system newaccount eosio --transfer alice $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer bob $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer carol $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer williamcurry $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer saganonroids $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer g4ydegenesis $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer haley $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer clara $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer christine $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer eunice $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer larry $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer janie $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer rod $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer becky $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer helen $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) system newaccount eosio --transfer chuck $(TELOS_PUBKEY) --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$(TECLOS) push action eosio.token transfer '[ "eosio", "eosstrawpoll", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "alice", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "bob", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "carol", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "williamcurry", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "saganonroids", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "g4ydegenesis", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "haley", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "clara", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "christine", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "eunice", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "larry", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "janie", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "rod", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "becky", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "helen", "2500.0000 SYS", "memo" ]' -p eosio
	$(TECLOS) push action eosio.token transfer '[ "eosio", "chuck", "2500.0000 SYS", "memo" ]' -p eosio
.PHONY: chain-accounts-telos

init-chain-telos: build chain-bios-telos chain-accounts-telos deploy-telos
.PHONY: init-chain-telos

$(DIST):
	mkdir -p $(DIST)

bash:
	docker run \
		--interactive \
		--tty \
		--rm \
		--entrypoint /bin/bash \
		--volume $(PWD):/contract:ro \
		--volume $(DIST):/dist \
		--volume $(PWD)/eosio.contracts:/eosio.contracts \
		sagansoftware/eos:v1.1.3
.PHONY: bash

start:
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	docker-compose up
.PHONY: start


build_docker:
	cd /build && \
		cmake /contract && \
		make && \
		./tests/unit_test --show_progress=yes
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
	$(CLEOS) set contract eosstrawpoll /build /build/eosstrawpoll.wasm /build/eosstrawpoll.abi

deploy-telos: build
	$(TECLOS) set contract eosstrawpoll /build eosstrawpoll.wasm eosstrawpoll.abi

optimize: build
	wasm-gc build/eosstrawpoll.wasm build/eosstrawpoll.wasm
	wasm-opt --output build/eosstrawpoll.wasm -Oz build/eosstrawpoll.wasm

generate-certs:
	openssl req -x509 -newkey rsa:4096 -keyout docker/key.pem -out docker/cert.pem -days 365 -subj '/CN=localhost' -nodes

jungle-info:
	$(CLEOS_JUNGLE) get info

deploy-jungle: optimize
	$(CLEOS_JUNGLE) set contract eosstrawpoll /build /build/eosstrawpoll.wasm /build/eosstrawpoll.abi
