#!/bin/sh
set -xe

./gen-svd.py > _.svd
svd2rust --target none -i _.svd >lib.rs
rm _.svd
rm -r src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt || true

sed -i 's/# ! \[ deny ( warnings ) \]//' src/lib.rs

rustup run nightly cargo doc
