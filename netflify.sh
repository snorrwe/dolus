#!/usr/bin/env bash

set -e

curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain nightly -y
source $HOME/.cargo/env

rustup target add wasm32-unknown-unknown

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cd dolus-web/app
npm i
npm run build
