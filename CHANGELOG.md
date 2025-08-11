<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.2] - 2025-08-11

### Fixed

- Fix feature flags leaking into generated code

## [0.5.1] - 2025-08-10

### Added

- Allow changing the type in `number!` macro

## [0.5.0] - 2025-08-09

### Added

- Add support for SeaORM

### Changed

- Upgrade to Rust edition 2024 and set MSRV to 1.85.0

## [0.4.3] - 2025-07-04

### Changed

- Publish crate using trusted publishing

## [0.4.2] - 2025-05-14

### Changed

- Restrict maximum dependency versions

## [0.4.1] - 2025-03-20

### Added

- Document the generated methods for newtypes

## [0.4.0] - 2025-02-16

### Added

- Allow to pass attributes to newtypes

### Fixed

- Include README in releases

## [0.3.0] - 2024-12-17

### Changed

- Unlock dependencies
- Bump Minimum Supported Rust Version
- Bump `secrecy` crate to 0.10.3

## [0.2.0] - 2024-08-21

### Added

- Create macro for fields with a URL
- Create macro for fields with a UUID
- Create macro for fields with a ULID

### Changed

- Build documentation for all features

## [0.1.0] - 2024-01-27

### Added

- Create `number!` macro for number-based fields
- Create `name!` macro for string-based fields
- Optionally derive serde traits
- Create `secret!` macro for fields with secrets

[0.5.2]: https://github.com/jdno/typed-fields/releases/tag/v0.5.2
[0.5.1]: https://github.com/jdno/typed-fields/releases/tag/v0.5.1
[0.5.0]: https://github.com/jdno/typed-fields/releases/tag/v0.5.0
[0.4.2]: https://github.com/jdno/typed-fields/releases/tag/v0.4.2
[0.4.1]: https://github.com/jdno/typed-fields/releases/tag/v0.4.1
[0.4.0]: https://github.com/jdno/typed-fields/releases/tag/v0.4.0
[0.3.0]: https://github.com/jdno/typed-fields/releases/tag/v0.3.0
[0.2.0]: https://github.com/jdno/typed-fields/releases/tag/v0.2.0
[0.1.0]: https://github.com/jdno/typed-fields/releases/tag/v0.1.0
