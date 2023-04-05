#!/bin/bash

set -xe

wit-bindgen rust --macro-call-prefix crate::qrdecoder:: \
                 --out-dir src \
                 --rustfmt \
                 qrdecoder.wit

cargo build --release --target=wasm32-unknown-unknown

rm -rf static
mkdir -p static
cp target/wasm32-unknown-unknown/release/qrdecoder.wasm static/qrdecoder_base.wasm

wasm-opt static/qrdecoder_base.wasm -O4 -o static/qrdecoder_opt.wasm

wasm-tools component new -o static/qrdecoder_comp.wasm static/qrdecoder_opt.wasm

jco transpile static/qrdecoder_comp.wasm \
              --name qrdecoder \
              --out-dir static \
              --optimize \
              --valid-lifting-optimization \
              --no-nodejs-compat

rm static/*.d.ts
rm -rf static/imports

wasm-tools strip -o static/qrdecoder.wasm static/qrdecoder.core.wasm

rm static/qrdecoder_*.wasm static/qrdecoder.core.wasm

ls -l static

# cp static/* ../nutria/apps/camera/qrdecoder/
