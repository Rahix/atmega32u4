#!/bin/sh
set -xe

./gen-svd.py > _.svd
svd2rust --target none -i _.svd
rm _.svd
rm -rf src/peripherals
form -i lib.rs -o src/peripherals 2>/dev/null
rm lib.rs
mv src/peripherals/lib.rs src/peripherals/mod.rs
cargo fmt || true

# Remove features and extern crates
sed -i '1d; 2d' src/peripherals/mod.rs

# Patch DEVICE_PERIPHERALS
sed -i 's/\(static mut DEVICE_PERIPHERALS\)/pub(crate) \0/' src/peripherals/mod.rs

# Generate interrupt macro
mkdir src/peripherals/interrupt
mv src/peripherals/interrupt.rs src/peripherals/interrupt/mod.rs
./int_macro.sh >src/peripherals/interrupt/vector_macro.rs
sed -i '1ipub mod vector_macro;' src/peripherals/interrupt/mod.rs

rustup run nightly cargo doc
