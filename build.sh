#!/bin/bash

set -xe

wit-bindgen js --export console.wit \
               --import qrdecoder_module.wit
rm *.d.ts
mv *.js static/

cargo build --release --target=wasm32-unknown-unknown

$WASM_OPT target/wasm32-unknown-unknown/release/qrdecoder.wasm \
    -O4 -o static/qrdecoder.wasm \
    --remove-imports

cp static/* ../nutria/apps/camera/qrdecoder/

ls -l static/*.wasm
