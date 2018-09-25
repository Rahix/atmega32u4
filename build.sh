#!/bin/sh
set -xe

./gen-svd.py > _.svd
svd2rust --target none -i _.svd >mod.rs
rm _.svd
rm -r src/peripherals
form -i mod.rs -o src/peripherals
rm mod.rs
mv src/peripherals/lib.rs src/peripherals/mod.rs
cargo fmt || true

# Remove features and extern crates
sed -i '1d; 2d' src/peripherals/mod.rs

# Patch DEVICE_PERIPHERALS
sed -i 's/\(static mut DEVICE_PERIPHERALS\)/pub(crate) \0/' src/peripherals/mod.rs
sed -i 's/\(interrupt::Interrupt\)/self::\0/' src/peripherals/mod.rs

rustup run nightly cargo doc
