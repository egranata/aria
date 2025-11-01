---
title: "What's New in Aria - October 2025"
description: "Aria edges closer to 1.0 with smarter REPLs, faster internals, and a more expressive standard library."
---

# What's New in Aria - October 2025

Aria is tightening up for 1.0. This month’s updates make the language cleaner and faster, at the REPL and in production code.

## Standard Library

* **Result everywhere** – The `Result` type now powers error handling across core libraries, including networking and JSON. Consistent, predictable, idiomatic.

* **Matrix indexing** – Access elements with `matrix[3, 4]` instead of `matrix.get(3, 4)!!`. Clearer, faster, and less boilerplate.

* **Path improvements** – `Path` now exposes creation, modification, and access times, and can compute the common ancestor of two paths.

* **String improvements** – `String.join` makes concatenation simpler and more flexible. `String.printf` combines formatting and printing in one call for concise, readable output.

* **setenv** – Modify environment variables directly from Aria.

## REPL

* **Smarter REPL** - When you forget a semicolon, the REPL won’t. It automatically terminates statements, so interactive sessions feel natural.

## Core Language

* **Faster hashing** — The VM now uses Rust’s hashing algorithm for better runtime performance on common operations.

* **Nested functions** - Define functions inside other functions, with identical semantics to lambdas.

* **Multiple index arguments** - `[]` and `[]=` now support multiple indices for clean multi-dimensional access.

* **Mixins/Extensions equivalence** - Members valid in an `extension` are now valid in a `mixin`, unlocking more flexible composition patterns.

* **ARIA_VERSION** - Access the current compiler and stdlib version directly for logging or diagnostics.

* **Type system root** - The type system now has a root type `Type`, which is the type of all types.

* **main with arguments** - `main(args)` or `main(...)` now receive command-line arguments automatically.

---

## Aria Release
* **Download Aria today** — The latest version [v0.9.20251013](https://github.com/egranata/aria/releases/tag/v0.9.20251013) is available on GitHub, with prebuilt binaries for Linux and macOS.

Prefer to build from source? Clone the [repo](https://github.com/egranata/aria) and get the freshest bits.

---
