#!/bin/bash
./target/debug/my-app &
export APP_PID=$!

sleep 1

MONGODB_HOST=$1 MONGODB_PORT=$2 cargo test

ERRCODE=$?

kill -SIGKILL $APP_PID

echo "CODE: ${ERRCODE}"

sleep 2

exit $ERRCODE