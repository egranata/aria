---
title: "What's New in Aria - September 2025"
description: "Explicit error handling with Result, cleaner object initialization syntax, and prebuilt binaries for Linux and macOS"
---

# What's New in Aria - September 2025

Quite a few improvements this month, mostly focused on cleaning up the language and the stanadard library. Here’s what’s new in September.

## Core language improvements

* **Result type** — Aria now has a [Result type](https://github.com/egranata/aria/pull/162) used to express success or failure of operations. Why it matters: no hidden exceptions, clearer control flow, easier composition with iterators and matches.

* **Object initializer syntax** — Initializers are more concise and flexible.

Shorthand: `.x` desugars to `.x = x` when `x` is in scope. You can mix field and index writes in one initializer.

---

## Aria Release
* **Download Aria today** — In addition to the usual build from source model, it is now possible to download pre-built binaries for Linux and macOS from the release page. Refer to [the release page](https://github.com/egranata/aria/releases) for details and to download the latest prebuilt today.

If you want the latest and greatest, you can still clone the [repo](https://github.com/egranata/aria) and build from source.

---
