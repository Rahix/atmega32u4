# Changelog

## [Unreleased]

## [0.1.2] - 2018-09-26
### Added
- Support for `interrupt!` macro to define custom interrupt handlers
- `USB` and `EXT_INT` peripherals
- *Internal*: Support for defining enumerated values in `toml` format
- *Internal*: Support for custom xml in `toml` format


## [0.1.1] - 2018-09-21
Bare minimum version, contains support for the following peripherals:

- [x] `PORTB`: Digital IO
- [x] `PORTC`: Digital IO
- [x] `PORTD`: Digital IO
- [x] `PORTE`: Digital IO
- [x] `PORTF`: Digital IO
- [x] `TIMER0`: 8-bit Timer/Counter0 with PWM
- [x] `TIMER1`: 16-bit Timer/Counter1
- [x] `TIMER3`: 16-bit Timer/Counter3
- [x] `TIMER4`: 10-bit High Speed Timer/Counter4
