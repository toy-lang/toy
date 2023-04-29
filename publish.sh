#!/bin/bash
cd ./toylang
cargo publish
cd ../toyc
cargo publish
cd ../toyvm
cargo publish