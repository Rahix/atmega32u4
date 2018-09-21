# `ATmega32U4`

Low level access to ATmega32U4 registers.  Refer to the ATmega32U4 datasheet for more detailed information.

## Implemented Peripherals
This crate not yet done and support for a lot of peripherals is still missing. The following have been
implemented already:

- [x] `PORTB`: Digital IO
- [x] `PORTC`: Digital IO
- [x] `PORTD`: Digital IO
- [x] `PORTE`: Digital IO
- [x] `PORTF`: Digital IO
- [x] `TIMER0`: 8-bit Timer/Counter0 with PWM
- [x] `TIMER1`: 16-bit Timer/Counter1
- [x] `TIMER3`: 16-bit Timer/Counter3
- [x] `TIMER4`: 10-bit High Speed Timer/Counter4

If support for a peripheral you need is missing, please open an issue or implement it yourself and send
a pull request ;)

## Generating this crate
This crate is built using [svd2rust]().  However, as there are no svd files available for *ATmega32U4*,
a python script is used to generate the svd file.  Take a look at `gen-svd.py` and `svd/peripheral.py`.

Peripherals can be specified in a toml format which should make adding new ones easier.  Use the existing
ones as reference (eg. `svd/timer0.svd`).

To actually perform a build, run `build.sh`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
