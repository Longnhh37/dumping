#!/bin/bash
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P)"
cd $SCRIPTPATH
cd ..

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/wasm_lib.wasm ./wasm_lib.wasm
wasm2wat ./wasm_lib.wasm -o ./wasm_lib.wat
