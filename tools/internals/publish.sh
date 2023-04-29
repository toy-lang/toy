#!/bin/bash
bash -e ../test.sh
cd ./toylang
cargo publish
cd ../toyc
cargo publish
cd ../toyvm
cargo publish