#!/bin/bash
bash -e ./tools/test.sh
cd ./toylang
cargo publish
cd ../toyc
cargo publish
cd ../toy_macros
cargo publish
cd ../toy_codegen
cargo publish
cd ../_toy_ast
cargo publish