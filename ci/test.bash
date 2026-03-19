#!/usr/bin/env bash
# Script for building your rust projects.
set -e

source ci/common.bash

# $1 {path} = Path to cross/cargo executable
CROSS=$1
# $1 {string} = <Target Triple>
TARGET_TRIPLE=$2

required_arg $CROSS 'CROSS'
required_arg $TARGET_TRIPLE '<Target Triple>'

# Pass through test database/amqp env vars so cross containers can use
# service containers instead of testcontainers (which need Docker-in-Docker).
CROSS_ENV_ARGS=""
if [ -n "$TEST_DATABASE_URL" ]; then
    CROSS_ENV_ARGS="$CROSS_ENV_ARGS -e TEST_DATABASE_URL=$TEST_DATABASE_URL"
fi
if [ -n "$TEST_AMQP_URL" ]; then
    CROSS_ENV_ARGS="$CROSS_ENV_ARGS -e TEST_AMQP_URL=$TEST_AMQP_URL"
fi

if [ -n "$CROSS_ENV_ARGS" ]; then
    export CROSS_CONTAINER_OPTS="${CROSS_CONTAINER_OPTS:-} $CROSS_ENV_ARGS"
fi

$CROSS test --target $TARGET_TRIPLE
$CROSS test --target $TARGET_TRIPLE --all-features
