#! /usr/bin/env bash

set -e

PUBKEY="EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"
PRIVKEY="5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3"
TELOS_PUBKEY="TLOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV"

function cleos() {
    docker-compose --file ./docker/docker-compose.yml exec keosd cleos \
        --url http://nodeosd:8888 \
        --wallet-url http://127.0.0.1:8900 \
        "$@"
}

# https://developers.eos.io/eosio-nodeos/docs/bios-boot-sequence
function setup_dev_chain() {
    cleos wallet create --to-console
    cleos wallet import --private-key $PRIVKEY
	cleos create account eosio eosio.bpay $PUBKEY $PUBKEY
	cleos create account eosio eosio.msig $PUBKEY $PUBKEY
	cleos create account eosio eosio.names $PUBKEY $PUBKEY
	cleos create account eosio eosio.ram $PUBKEY $PUBKEY
	cleos create account eosio eosio.ramfee $PUBKEY $PUBKEY
	cleos create account eosio eosio.saving $PUBKEY $PUBKEY
	cleos create account eosio eosio.stake $PUBKEY $PUBKEY
    cleos create account eosio eosio.token $PUBKEY $PUBKEY
	cleos create account eosio eosio.vpay $PUBKEY $PUBKEY
	cleos create account eosio eosio.rex $PUBKEY $PUBKEY
	cleos set contract eosio.token contracts/eosio.token
	cleos set contract eosio.msig contracts/eosio.msig
	cleos push action eosio.token create '[ "eosio", "1000000000.0000 SYS" ]' -p eosio.token
	cleos push action eosio.token issue '[ "eosio", "1000000000.0000 SYS", "memo" ]' -p eosio
	cleos set contract eosio contracts/eosio.system
    cleos push action eosio setpriv '[ "eosio.msig", 1 ]' -p eosio@active
    cleos push action eosio init '[ 0, "4,SYS" ]' -p eosio
}

function setup_dev_accounts() {
    cleos system newaccount eosio --transfer dappcontract $PUBKEY \
        --stake-net "100000.0000 SYS" \
        --stake-cpu "100000.0000 SYS" \
        --buy-ram-kbytes 8192
}

function deploy_contract() {
    cleos set contract \
        dappcontract \
        contracts/dapp.system \
        contract.wasm \
        contract.abi
}

setup_dev_chain
setup_dev_accounts
deploy_contract