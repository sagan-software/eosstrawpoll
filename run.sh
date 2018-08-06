#!/bin/bash

NPM=$(npm bin)
PWD=$(pwd)
PUBKEY=EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY=5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

export APP=eosstrawpoll
export SRC=src
export DIST=dist

function run_task {
    task=$1
    printf ">>>>>>>> Running task '%s' \\n" "$task"
    run_${task} "$@"
    case "$?" in
        127)
            printf "!!!!!!!! Error: unknown task '%s'. Exiting now \\n" "$task"
            exit 1
            ;;
        1)
            printf "!!!!!!!! Error running task '%s'. Exiting now \\n" "$task"
            exit 1
            ;;
    esac
}

function command_exists {
    type "$1" &> /dev/null ;
}

function run_install {
    git submodule update --init --recursive
    if ! command_exists rustup; then
        curl https://sh.rustup.rs -sSf | sh
    fi
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	rustup component add rustfmt-preview --toolchain nightly
	cargo install --force cargo-watch
	cargo install --force cargo-web
    if command_exists yarn; then
	    yarn install
    elif command_exists npm; then
        npm install
    else
        printf "!!!!!!!! Error: yarn and npm could not be found. Exiting now \\n"
        exit 1
    fi
}

function run_clean {
    mkdir -p $DIST
    rm -f $DIST/*
}

function run_build_cargo {
    cargo web deploy \
        --target=wasm32-unknown-unknown \
        --release \
        --verbose
}

function run_watch_cargo {
    cargo watch \
        -i '*.css' \
        -i '*.html' \
        -i '*.js*' \
        -i '*.sh' \
        -i 'node_modules' \
        -x "web deploy --target=wasm32-unknown-unknown --release --verbose"
}

function run_build_webpack {
    $NPM/webpack $@
}

function run_watch_webpack {
    run_build_webpack --watch
}

function run_build_css {
    $NPM/postcss \
        static/index.css \
        --no-map \
        --output $DIST/index.css \
        --use postcss-import \
        --use postcss-preset-env \
        --use cssnano \
        $@
}

function run_watch_css {
    run_build_css --watch
}

function run_build_website {
    run_task "build_cargo"
    run_task "build_webpack"
    run_task "build_css"
}

function run_build_contract {
    docker-compose exec nodeosd /build_contracts.sh
}


CLEOS="docker-compose exec nodeosd cleos --url http://nodeosd:8888 --wallet-url http://localhost:8888"

function run_create_accounts {
	$CLEOS system newaccount eosio --transfer eosstrawpoll $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 2000
	$CLEOS system newaccount eosio --transfer alice $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$CLEOS system newaccount eosio --transfer bob $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$CLEOS system newaccount eosio --transfer carol $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$CLEOS system newaccount eosio --transfer williamcurry $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$CLEOS system newaccount eosio --transfer saganonroids $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
	$CLEOS system newaccount eosio --transfer g4ydegenesis $PUBKEY --stake-net "100000.0000 SYS" --stake-cpu "100000.0000 SYS" --buy-ram-kbytes 512
}

function run_deploy_contract {
    $CLEOS set contract eosstrawpoll \
        /eos/contracts/eosstrawpoll \
        /eos/contracts/eosstrawpoll/eosstrawpoll.wast \
        /eos/contracts/eosstrawpoll/eosstrawpoll.abi && \
        $CLEOS get table eosstrawpoll alice polls
}


function run_clean_docker {
    docker-compose down
    docker kill $(docker ps -q)
}

function run_full_clean_docker {
    run_clean_docker
    docker rm $(docker ps -a -q)
    docker rmi $(docker images -q)
    docker system prune
}

function run_setup_chain {
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
}

function run_cleos {
    if [ "$(docker ps -q -f name=nodeosd)" ]; then
        docker-compose exec nodeosd "$@"
    else
        echo "Please start Docker first with './run.sh start_docker'"
    fi
}

function run_build {
    export NODE_ENV=production
    run_clean
    run_build_website
    run_build_contract
}

function run_start_server {
    $NPM/browser-sync start \
        --port 1337 \
        --https \
        --single \
        --server dist \
        --no-open \
        --no-ui \
        --watch
}

function run_start_website {
	$NPM/concurrently \
        --raw \
        --kill-others \
		"./run.sh watch_cargo" \
		"./run.sh watch_webpack" \
		"./run.sh watch_css" \
        "./run.sh start_server"
}

function run_start {
    run_task "clean"
    run_task "start_website"
}

run_task "$@"