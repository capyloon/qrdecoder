#!/bin/bash

set -xe

wit-bindgen host js --export console.wit \
                    --import qrdecoder_module.wit
rm *.d.ts
mv *.js static/

cargo build --release --target=wasm32-unknown-unknown

wasm-bindgen \
    --target web \
    --out-dir static \
    target/wasm32-unknown-unknown/release/qrdecoder.wasm

wasm-opt static/qrdecoder_bg.wasm \
    -O4 -o static/qrdecoder.wasm

rm static/*.d.ts
rm static/qrdecoder.js # wasm-bindgen generated that we don't need.
rm static/qrdecoder_bg.wasm

mv static/qrdecoder-module.js static/qrdecoder_module.js
cp static/* ../nutria/apps/camera/qrdecoder/

ls -l static/*.wasm
