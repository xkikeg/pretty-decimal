# Change Log

## [Unreleased] - ReleaseDate

### Added

### Changed

* `ParseError` has a new variant `UnexpectedEnd`. As the enum isn't `#[non_exhaustive]`, this is a
  breaking change for code matching on it exhaustively.

### Fixed

* `FromStr` rejected malformed numbers it used to accept:
  * no digit at all: `""`, `"-"`, `"+"`, `"."`, `"-."` were parsed as 0.
  * a second `.`: `"1..2"` was parsed as 1.2, and `"1.2.3"` as 12.3.
  * an incomplete digit group after a comma: `"1,"` was parsed as 1, `"1,23"` as 123, and
    `"12,34"` as 1234.

## [0.2.1] - ReleaseDate

### Fixed

* Supported Rust version 1.79.0, also lowered edition to 2018.

## [0.2.0] - 2025-07-18

### Added

* New API to construct value with format (https://github.com/xkikeg/pretty-decimal/pull/10).
* Supported formatting options (https://github.com/xkikeg/pretty-decimal/pull/7).

### Changed

* Renamed `Error` type to `ParseError`.

## [0.1.2] - 2025-07-11

### Changed

* Use 2021 edition instead of 2024.

## [0.1.1] - 2025-07-10

Initial release of the crate.
