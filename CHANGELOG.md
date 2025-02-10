# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added project componenets for the Arduino Uno R3 hardware from the [avr-hal dependency](https://github.com/rahix/avr-hal)
    - avr-specs/avr-atmega328p.json
    - Cargo.toml
    - rust-toolchain.toml
    - .cargo/config.toml
- Added ```time``` module to abstract away timekeeping componenets like hardware clocks into a Timer struct etc.
- Added ```led``` module to abstract away digital output components into a LedArray and LedTask structs.
- Added ```button``` module to abstract away digital input components into a ButtonTask struct.
- Added ```channel``` module to abstract away safe sharing of mutable data between tasks that need to be interactive.
- Added [README.md]
- Added [LICENSE.md] to comply with dependencies
- Added this [CHANGELOG.md]
