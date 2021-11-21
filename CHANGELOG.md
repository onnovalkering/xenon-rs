# Changelog

All notable changes to `xenon-rs` will be documented in this file.

## [0.4.3] - 2021-11-21
## Changed
- Compute methods accept `Into<Job>` where possible.

## [0.4.2] - 2021-11-19
## Fixed
- Missing fmt argument in error message ([#1](https://github.com/onnovalkering/xenon-rs/pull/1))

## Changed
- Updated dependencies to the latest version.
- Use fully qualified names for third-party images in Dockerfiles.

## [0.4.1] - 2021-08-11
### Changed
- Disable formatting of generated gRPC code.

## [0.4.0] - 2021-08-08
### Added
- Added missing Xenon functions to compute and storage modules.

### Changed
- Use `tonic` and `prost` for gRPC implementation.
- Use `Into<T>` instead of `T` where possible.

## [0.3.0] - 2021-03-03
### Added
- Support for macOS (build, generate gRPC interface, and CI).

### Changed
- Replaced `failure` with `anyhow` for error handling.
- Library is now exposed as `xenon`, instead of `xenon_rs`.

## [0.2.0] - 2020-03-04
### Changed
- Generated gRPC interface is now included in source code.

## [0.1.0] - 2020-03-02
### Added
- Initial implementation.
