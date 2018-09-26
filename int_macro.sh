#!/bin/sh

echo '#[doc(hidden)]'
echo '#[macro_export]'
echo 'macro_rules! __interrupt_vector {'
sed '/Interrupt::.\+ =>/!d
s/ \+Interrupt::\(.\+\) => \(.\+\),/    (\1, $b:block) => {#[no_mangle] pub unsafe extern "avr-interrupt" fn __vector_\2() {$b}};/' src/peripherals/interrupt/mod.rs
echo '}'
