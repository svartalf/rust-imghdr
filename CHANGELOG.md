# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.7.0] - 2019-05-08
### Added
 - `from_reader` function which allows to determine image format from the generic `T: Read` argument

### Changed
 - Crate was updated to Rust edition 2018 (minimal rustc version is `1.31` now)
 - Crate was re-licensed under the dual `Apache 2.0` or `MIT` license
 - `open` function was renamed into `from_file`, it returns `Result<Option<Type>>` now
 - `what` function was renamed into `from_bytes`
 - `from_file` and `from_reader` functions are available only with `std` feature (enabled by default)
 - `from_bytes` function can be used in the `no_std` environments

## [0.6.0] - 2019-05-07
### Added
 - Support for EXR and HDR (RGBE) formats ([#2](https://github.com/svartalf/rust-imghdr/pull/2))

## [0.5.0] - 2019-05-06
### Added
 - Support for ICO format ([#1](https://github.com/svartalf/rust-imghdr/pull/1))

## [0.4.0] - 2016-07-20
### Added
 - Support for FLIF format

## [0.3.0] - 2016-07-20
### Added
 - Support for SGI (RGB) format

## [0.2.0] - 2016-07-20
### Added
 - Support for PBM, PGM and PPM formats
 - Changelog!
