#! /usr/bin/env bash

set -e

function build_contract_outside_container() {
    printf "\\t=========== Building Docker container ===========\\n\\n"
    docker build \
        --memory 10G \
        --tag sagan-software/eos:latest \
        --file ./docker/eos.dockerfile \
        ./docker
    docker run \
        --interactive \
        --tty \
        --rm \
        --volume /$(PWD)/contract/src:/contract/src:ro \
        --volume /$(PWD)/contract/eosio.contracts:/contract/eosio.contracts:ro \
        --volume /$(PWD)/contract/tests:/contract/tests:ro \
        --volume /$(PWD)/contract/CMakeLists.txt:/contract/CMakeLists.txt:ro \
        --volume /$(PWD)/contract/build:/contract/build \
        --volume /$(PWD)/build.sh:/contract/build.sh:ro \
        --workdir //contract \
        sagan-software/eos:latest \
        bash build.sh contract_inner
}

function build_contract_inside_container() {
    printf "\\t=========== Building contracts ===========\\n\\n"

    RED='\033[0;31m'
    NC='\033[0m'

    CORES=`getconf _NPROCESSORS_ONLN`
    mkdir -p build
    pushd build &> /dev/null
    cmake ../
    make -j${CORES}
    ./tests/unit_test --show_progress=yes
    popd &> /dev/null
}

INPUT=$1

if [[ $INPUT == "contract" ]]; then
    build_contract_outside_container
elif [[ $INPUT == "contract_inner" ]]; then
    build_contract_inside_container
else
    printf "Invalid input: %s\\n" $INPUT 1>&2
    exit 1
fi
