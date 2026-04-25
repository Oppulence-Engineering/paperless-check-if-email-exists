#!/usr/bin/env bash
# Script for testing your rust projects.
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
if [ -n "$TEST_DATABASE_URL" ] || [ -n "$TEST_AMQP_URL" ]; then
    CROSS_ENV_ARGS="$CROSS_ENV_ARGS -e USE_LOCAL_TEST_INFRA=1"
fi

if [ -n "$CROSS_ENV_ARGS" ]; then
    export CROSS_CONTAINER_OPTS="${CROSS_CONTAINER_OPTS:-} $CROSS_ENV_ARGS"
fi

# The CLI matrix runs through cross containers. The backend suite includes
# resilience tests that intentionally spawn Docker-owned Postgres/RabbitMQ
# containers, and those containers do not have a Docker CLI available. The main
# PR workflow still runs the backend suite on a native Linux runner with Docker.
TEST_SCOPE=(--workspace)
if [ "$(basename "$CROSS")" != "cargo" ]; then
    echo "cross-based CLI test job - excluding reacher_backend integration tests"
    TEST_SCOPE+=(--exclude reacher_backend)
elif [ -z "$TEST_DATABASE_URL" ] && ! docker info > /dev/null 2>&1; then
    echo "Docker not available and TEST_DATABASE_URL not set - excluding reacher_backend integration tests"
    TEST_SCOPE+=(--exclude reacher_backend)
fi

$CROSS test "${TEST_SCOPE[@]}" --target $TARGET_TRIPLE
$CROSS test "${TEST_SCOPE[@]}" --target $TARGET_TRIPLE --all-features
