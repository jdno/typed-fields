<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.3.0]: https://github.com/jdno/typed-fields/releases/tag/v0.3.0
[0.2.0]: https://github.com/jdno/typed-fields/releases/tag/v0.2.0
[0.1.0]: https://github.com/jdno/typed-fields/releases/tag/v0.1.0
