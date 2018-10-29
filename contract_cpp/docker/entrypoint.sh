#!/bin/sh

ARGS=$(eval echo "$@")

if [ ! "$ARGS" ]; then
    echo "[entrypoint.sh] empty arguments are not allowed"
    exec nodeos --help
    exit 1
else
    echo "[entrypoint.sh] received arguments: $@"
fi

if [ "$STARTUP_DELAY" ]; then
    echo "[entrypoint.sh] STARTUP_DELAY is set to $STARTUP_DELAY. waiting..."
    sleep $STARTUP_DELAY
    echo "[entrypoint.sh] done waiting"
fi

mkdir -p $DATA_DIR
mkdir -p $CONFIG_DIR

if [ ! -f "$CONFIG_DIR/config.ini" ]; then
    echo "[entrypoint.sh] $CONFIG_DIR/config.ini does not exist, using default"
    cp /config.ini $CONFIG_DIR
fi

echo "[entrypoint.sh] nodeos $ARGS"

exec nodeos $ARGS