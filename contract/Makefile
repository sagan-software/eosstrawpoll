PWD := `pwd`
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3
DOCKER := docker-compose exec nodeosd
CLEOS := $(DOCKER) cleos --url http://nodeosd:8888 --wallet-url http://localhost:8888
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

$(DIST):
	mkdir -p $(DIST)

build: $(DIST)
	# $(EOSIOCPP) \
	# 	--outname /contracts/eosstrawpoll/eosstrawpoll.wast \
	# 	/contracts/eosstrawpoll/eosstrawpoll.cpp
	docker run \
		--interactive \
		--tty \
		--rm \
		--entrypoint eosio-cpp \
		--volume $(PWD):/contract:ro \
		--volume $(DIST):/dist \
		sagansoftware/eos:v1.1.3 \
		-I=/contract/include \
		-I=/root/opt/boost/include \
		-o=/dist/test2.wasm \
		-O=3 \
		-lto-opt=O3 \
		-fmerge-all-constants \
		-fstrict-return \
		/contract/src/eosstrawpoll.cpp
.PHONY: contract

test: $(DIST)
	$(DOCKER) /build_contracts.sh
.PHONY: test-contract