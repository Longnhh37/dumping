#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ../..
cd frontend
npm install
npm run build
cd ../ingress
cargo clean
cargo run
