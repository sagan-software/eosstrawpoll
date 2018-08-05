#!/bin/bash

NPM=$(npm bin)
PWD=$(pwd)

export APP=eosstrawpoll
export SRC=src
export DIST=dist

function run_task {
    task=$1
    printf ">>>>>>>> Running task '%s' \\n" "$task"
    run_${task}
    case $? in
        127)
            printf "!!!!!!!! Error: unknown task '%s'. Exiting now \\n" "$task"
            exit 1
            ;;
        1)
            printf "!!!!!!!! Error running task '%s'. Exiting now \\n" "$task"
            exit 1
            ;;
        *)
            printf ">>>>>>>> Successfully ran task '%s' \\n" "$@"
            ;;
    esac
}

function command_exists {
    type "$1" &> /dev/null ;
}

function run_install {
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
        --release
}

function run_watch_cargo {
    cargo watch \
        -i '*.css' \
        -i '*.html' \
        -i '*.js*' \
        -i '*.sh' \
        -i 'node_modules' \
        -x "web deploy --target=wasm32-unknown-unknown --release"
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
    docker run \
        --interactive \
        --tty \
        --rm \
        --volume $PWD/build_contracts.sh:/build_contracts.sh \
        --volume $PWD/contract:/contracts/eosstrawpoll \
        --entrypoint /build_contracts.sh \
        sagansoftware/eos:v1.1.0
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

run_task $@