#!/bin/bash

set -xe

wit-bindgen host js --export console.wit \
                    --import qrdecoder_module.wit
rm *.d.ts
mv *.js static/

cargo build --release --target=wasm32-unknown-unknown

wasm-opt target/wasm32-unknown-unknown/release/qrdecoder.wasm \
    -O4 -o static/qrdecoder.wasm \
    --remove-imports

mv static/qrdecoder-module.js static/qrdecoder_module.js
cp static/* ../nutria/apps/camera/qrdecoder/

ls -l static/*.wasm
