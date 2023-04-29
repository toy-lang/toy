#!/bin/bash
bash -e ./tools/test.sh
cd ./toylang
cargo publish
cd ../toyc
cargo publish
cd ../toyvm
cargo publish