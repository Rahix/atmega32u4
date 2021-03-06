# Changelog

## [Unreleased]
### Fixed
- Adjust `build.sh` to work with svd2rust 0.14.0


## [0.1.3] = 2018-10-1
### Changed
- Made a few registers safe to write

### Added
- Enumerated values for `Compare Output Mode` & `Clock Source`
- *Internal*: Add `safe` flag for registers and fields


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
