# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
For information on Aria's versioning scheme and release policy refer to [our Release Policy](https://arialang.github.io/release_policy.html).

## [0.9.20251118]

### Added

- Add support for `??` and `!!` operators to `Maybe` objects
- Support for skipping and truncating iterators
- `SipHash` support has been added to the standard library
- Benchmarking utilities have been introduced to measure performance of code snippets
- `List` can now be hashed if all its values are hashable
- `Path.glob`
- Initial support for widgets as a code organizing structure
- Draft language server protocol (LSP) support for better IDE integration
- It is now possible to write `x,y = y,x` for swapping values (multiple assignment) and to declare multiple variables in one statement (`val x = 1, y = 2;`)
- `Map.frequency_map` has been added to create frequency maps from iterables
- Intersection types (`TypeA & TypeB`) have been introduced

### Fixed

- `Map` would error if an object's `hash` returned a negative value; it now handles this correctly
- It is now possible to write to a captured value without reading it
- `isa` now works with mixins

### Changed

- `Path.entries()` returns an iterator instead of a list for better performance
- `Path` operations consistently return `Maybe` / `Result` types for improved error handling
- `Iterator` uses `Maybe` instead of `.done` for its iteration protocol
- Hexadecimal literals are treated as unsigned integers
- Aria now lives under the `arialang` GitHub organization (https://github.com/arialang/aria)

### Deprecated

None

### Removed

None
