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

The person cutting the release should make sure that the `CHANGELOG.md` file is updated with the changes that went into the release. This includes new features, bug fixes, and any other relevant information. The format and structure of the changelog is inspired by [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), but contributors and maintainers should exercise their best judgement as to the specific format and content of the file.

If all these steps have been followed, the release can be cut by a maintainer by updating the version number in the Cargo manifest files, committing the changes to the main branch, and tagging the release with the `v<...>` tag (e.g. `0.9.20251010`) and pushing the tag to GitHub. This will automatically trigger the creation of a release on GitHub, which will build the release artifacts and make them available for download.

## 4. When to Release

Ideally, an Aria release would be cut once a month, and the latest release would be published on the Aria blog, alongside the changes that went into it, and any credits to the contributors that made it possible.

If an Aria release has to be cut more frequently, e.g. to address a critical bug, this is acceptable. It should be documented on the Aria blog and the previous blog post should be updated to reflect the new release.

## 5. Release Branches and Maintenance

**This policy does not enter into effect (and is itself subject to change) until Aria 1.0 enters its stabilization cycle, as defined below.**

Once all open issues in the Aria 1.0 milestone are "bugs" and no new feature development is being considered, the first stable branch will be created. This branch will be called `release/1.0` and will be used to cut Aria 1.0 releases. Development of new features will continue on the `master` branch, which will eventually lead to Aria 1.5 and beyond.

A commit is allowed on the stable branch only if it is a bug fix or a documentation change. Commits must first go to `master` and then be cherry-picked to the stable branch, if applicable to it. Maintainers must use `git cherry-pick -x` to preserve origin reference and traceability.

Once the first Aria 1.0 release is cut, `release/1.0` will become the ongoing maintenance branch for Aria 1.0 releases. For example, if a bug is discovered in Aria `1.0.20251225`, a fix will be made on `master` (if necessary), cherry-picked to `release/1.0`, and a new release, e.g. `1.0.20260115`, will be cut from it. Aria 1.0 may take a while to converge once the first release is cut (e.g. more bugs are discovered during testing). This is OK. More versioned 1.0 releases will be cut until sufficient stability is reached. At this point, Aria 1.0 will be officially announced on the Aria blog, with the "golden" release announced on the Aria blog.

In a similar vein, once Aria 1.5 is ready to converge (i.e. only bugs remain in the milestone), `release/1.5` will be created from `master`, and the process will repeat. Note that `release/1.0` will remain available for bug fixes to Aria 1.0 releases, if necessary.

While Aria N.x is under development, the recent-most N.(x-1) version will continue to receive bug fixes as necessary. So, for example, if a bug is identified in Aria 1.5 that would also affect Aria 1.0, a fix will be made on `master`, cherry-picked to `release/1.5`, and to `release/1.0`, and a new Aria 1.0 release will be cut. If a hypothetical Aria 1.7 release were to follow 1.5, bugs would be fixed in `master`, `release/1.7`, `release/1.5`, but **not** in `release/1.0`.

Once Aria (N+1).0 is released, the **latest** Aria N.x release will enter a frozen stage. For at least 3 months following, the frozen release will receive only critical bug fixes. The end of this timeline will be announced on the Aria blog at least 1 month in advance. Previous Aria N.x releases will **not** receive any further updates.

For the purposes of this policy, critical is defined as:
- security vulnerabilities;
- regressions that break existing code.

At any point in time, a git tag `latest-N.x` will point to the latest release in the N.x series. This is to allow users to easily find the latest release in a given series. Note that individual releases will continue to be tagged with the `v<...>` tag (e.g. `v1.0.20251225`) and these tags will never move. The `latest-N.x tag` is force-updated on each new release.

Contributors are generally not expected to do anything beyond submit their pull requests (see [Contributing Guide](CONTRIBUTING.md)). Cherry-picking and cutting releases is the responsibility of the maintainers. However, they may be asked to help disambiguate conflicts or other issues that arise during the cherry-pick process.

Users are encouraged to migrate their code to the latest released Aria version. For example, once Aria 1.5 is released, Aria 1.0 will enter the frozen stage, and users will be encouraged to migrate to Aria 1.5. Once Aria 2.0 is released, Aria 1.5 will enter the frozen stage, and users will be encouraged to migrate to Aria 2.0.

The overarching goal of these policies is to always have two actively maintained Aria versions: the latest one, and the previous one. This allows users to have a stable version to fall back to, while still being able to take advantage of the latest features and improvements in the language.

### 5.1 Summary Table

| Stage | Branch | Allowed Commits | Receives Fixes For | Typical Lifetime | Maintainer Actions |
|--------|---------|-----------------|--------------------|------------------|--------------------|
| **Development** | `master` | New features, refactors, and bug fixes | Upcoming major release | Ongoing | Merge PRs, tag alpha/beta milestones |
| **Maintenance** | `release/N.x` | Bug fixes and documentation updates | Current and prior minor releases | Until (N+1).0 release | Cherry-pick fixes from `master`, cut patch releases |
| **Frozen** | `release/N.x` | Critical fixes only (security, regressions) | Previous major release | ~3 months post new major | Limited backports, announce freeze and EOL timeline |
| **EOL** | — | None | — | Indefinite | Branch remains archived, no updates |
