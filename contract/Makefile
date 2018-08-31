PWD := `pwd`
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
DOCKER := docker-compose exec nodeosd
CLEOS := $(DOCKER) cleos --url http://127.0.0.1:8888 --wallet-url http://127.0.0.1:8888
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

optimize: build
	wasm-gc build/eosstrawpoll.wasm build/eosstrawpoll.wasm
	wasm-opt --output build/eosstrawpoll.wasm -Oz build/eosstrawpoll.wasm

generate-certs:
	openssl req -x509 -newkey rsa:4096 -keyout docker/key.pem -out docker/cert.pem -days 365 -subj '/CN=localhost' -nodes