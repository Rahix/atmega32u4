#!/bin/sh
set -xe

./gen-svd.py > _leonardo.svd
svd2rust --target none -i _leonardo.svd >lib.rs
rm _leonardo.svd
rm -r src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt || true

sed -i 's/# ! \[ deny ( warnings ) \]//' src/lib.rs

rustup run nightly cargo doc
