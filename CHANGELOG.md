# Changelog (disco-server)

## 1.3.0 - 2024-12-06

- Added support for writing register values with GDB

## 1.2.1 - 2020-04-29

### Changed

- `--debug` flag now prints out CLI arguments as well

## 1.2.0 - 2020-04-29

### Changed

- Removed `--audio` flag (see emulator 1.2.0 change).
- Version info now sourced from Cargo.toml file. Fixes inconsistent `--version` results.

## 1.1.0 - 2020-04-03

### Added

- CLI arg `--samples <start> <count>` to print a list of `count` audio samples starting from the `start` sample. This arg superceeds all others (except `--version`).



## 1.0.0 - 2020-03-15

### Added

- Change log started
