# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `??` and `!!` now also work with `Maybe`
- `SipHash-1-3` support
- `List` is now hashable if its contents are
- `Path.glob` method to find files matching a pattern

### Fixed

- If an object `hash` returns a negative integer, `Map` will not crash when using it as a key.
- It is possible to write to an captured value in a closure before reading it. Before this was a compile-time error.

### Changed

- `Path.entries` now returns an iterator instead of a list, improving performance for large directories.
- `Path` methods consistently return `Maybe`/ `Result` types to better handle errors.
- Iterators now use `Maybe` to signal end-of-iteration instead of ad-hoc `done` values.

### Deprecated

- Deprecated features go here

### Removed

- If you remove anything, it goes here
