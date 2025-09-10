---
title: "Aria Release Policy"
description: "Guidelines for releasing new versions of Aria."
---

# Aria Release Policy

## 1. Introduction

This document provides guidance and instructions for releasing new versions of the Aria programming language. It outlines the steps to be taken, the criteria for versioning, and the responsibilities of maintainers.

If you are not a maintainer, please refer to the [Contributing Guide](CONTRIBUTING.md) for information on how to contribute to Aria.

## 2. Versioning

Aria is currently versioned at 0.9, indicating that the language is still under development, which means that:

- the language may change in incompatible ways;
- the standard library may change in incompatible ways;
- features may be added or removed at any time with no process for deprecation;
- there is no guarantee of stability or readiness for production use.

Aria will reach version 1.0 once all of the following criteria are met:

- all issues marked as "Aria 1.0" in the [issue tracker](https://github.com/egranata/aria/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22Aria%201.0%20Roadmap%22) are resolved;
- the criteria defined in the [roadmap](ROADMAP.md) for the "Very-Short-Term Goal" are met;
- the core language and standard library are considered stable and usable.

Once Aria version 1.0 is reached, we will only consider breaking changes on a *must* basis, i.e. we will only break existing code if there is no other realistic path to achieve a significant language goal.

Aria releases will be versioned using the language's own version and a date marker in the format `YYYYMMDD`, e.g. `0.9.20260613`. It is not expected that multiple releases will be cut in the same day, but if they are, an increasing positive integer patch counter will be added, e.g. `0.9.202606131`, `0.9.202606132`, etc.

## 3. Release Process

Before releasing a new version of Aria, ensure that all tests pass and that the codebase is in a stable state. If any new features or bug fixes have been added, they should be clearly documented in the appropriate documents.

If all these steps have been followed, the release can be cut by a maintainer by updating the version number in the Cargo manifest files, committing the changes to the main branch, and tagging the release with the `v<...>` tag (e.g. `0.9.20251010`) and pushing the tag to GitHub. This will automatically trigger the creation of a release on GitHub, which will build the release artifacts and make them available for download.

## 4. When to Release

Ideally, an Aria release would be cut once a month, and the latest release would be published on the Aria blog, alongside the changes that went into it, and any credits to the contributors that made it possible.

If an Aria release has to be cut more frequently, e.g. to address a critical bug, this is acceptable. It should be documented on the Aria blog and the previous blog post should be updated to reflect the new release.

## 5. Stable Branch

Prior to Aria 1.0, there is no stable branch. All development happens on the `master` branch, and releases are cut from it. Once Aria reaches version 1.0, a stable process will be defined and this document updated to reflect it.

Upon reaching stability, the language itself may define appropriate mechanism for version and feature discovery, as well as a process for deprecation and eventual removal of features, and APIs. This will be documented in the language specification and the standard library documentation.
