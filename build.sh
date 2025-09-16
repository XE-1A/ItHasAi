#!/usr/bin/env bash
set -euxo pipefail

cd $(dirname $0)
rm -rf out/**
rm -f itHasAi.zip

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "itHasAi" \
    ./target/wasm32-unknown-unknown/release/itHasAi.wasm

cp web/** out

pushd out
zip -r ../itHasAi.zip .
popd