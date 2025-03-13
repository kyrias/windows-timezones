# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Changed

- BREAKING: Upgraded all dependencies. ([#14](https://github.com/kyrias/windows-timezones/pull/14))
  - `chrono-tz` upgraded from 0.9 to 0.10.  Support for 0.9 is retained under the `chrono-tz-0_9` feature.
  - `strum` upgraded from 0.26 to 0.27.  The derive macros are mutually incompatible so support for the old version could not be retained.


## [0.4.0]

### Changed

- BREAKING: Upgraded all dependencies. ([#13](https://github.com/kyrias/windows-timezones/pull/13))


## [0.3.1]

This release upgrades to release 45 of the CLDR data.

### Added

- Added `WindowsTimezone::name` method and `Display` implementation.  ([#12](https://github.com/kyrias/windows-timezones/pull/12))
- Implemented `TryFrom<chrono_tz::Tz>` for `WindowsTimezone` under the `chrono-tz` feature.  ([#11](https://github.com/kyrias/windows-timezones/pull/11))

### Changed

- Central Asia Standard Time now corresponds to the tzdb ID Asia/Bishkek instead of Asia/Almaty.  ([CLDR-17351])

[CLDR-17351]: https://github.com/unicode-org/cldr/pull/3498


## [0.3.0]

### Changed

- Bumped SQLx dependency from 0.6.3 to 0.7.  ([#9](https://github.com/kyrias/windows-timezones/pull/9), [#10](https://github.com/kyrias/windows-timezones/pull/10))
- Removed minor dependency bounds from crate.  These serve no purpose for us. ([#10](https://github.com/kyrias/windows-timezones/pull/10))


## [0.2.1]

### Added

- `FromStr` implementation for the `WindowsTimezone` type.  ([#5](https://github.com/kyrias/windows-timezones/pull/5), [#6](https://github.com/kyrias/windows-timezones/pull/6))


## [0.2.0]

### Changed

- `strum` dependency upgraded from 0.24.1 to 0.25.0.


## [0.1.1]

This release upgrades to release 43 of the CLDR data.

### Changed

- Mountain Standard Time (Mexico) now corresponds to the tzdb ID America/Mazatlan instead af America/Chihuahua. ([CLDR-16386](https://github.com/unicode-org/cldr/pull/2716))


[Unreleased]: https://github.com/kyrias/windows-timezones/compare/0.4.0...main
[0.4.0]: https://github.com/kyrias/windows-timezones/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/kyrias/windows-timezones/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/kyrias/windows-timezones/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/kyrias/windows-timezones/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/kyrias/windows-timezones/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/kyrias/windows-timezones/compare/0.1.0...0.1.1
