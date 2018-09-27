#!/bin/bash

CLEOS="docker-compose exec nodeosd cleos --url http://nodeosd:8888 --wallet-url http://keosd:8900"

PUBKEY=EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY=5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

$CLEOS wallet create
$CLEOS wallet import --private-key $PRIVKEY
$CLEOS create account eosio eosio.token $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.msig $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.ram $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.ramfee $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.stake $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.bpay $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.vpay $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.saving $PUBKEY $PUBKEY
$CLEOS create account eosio eosio.names $PUBKEY $PUBKEY
$CLEOS set contract eosio.token /eos/build/contracts/eosio.token
$CLEOS set contract eosio.msig /eos/build/contracts/eosio.msig
$CLEOS push action eosio.token create '[ "eosio", "10000000000.0000 SYS" ]' -p eosio.token
$CLEOS push action eosio.token issue '[ "eosio", "10000000000.0000 SYS", "memo" ]' -p eosio
$CLEOS set contract eosio /eos/build/contracts/eosio.system
# cleos push action eosio setpriv '["eosio.msig", 1]' -p eosio@active