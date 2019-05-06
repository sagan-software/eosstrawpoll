#! /usr/bin/env bash

set -e

docker-compose --file ./docker/docker-compose.yml down
docker volume rm nodeos-data-volume
docker volume rm keosd-data-volume
