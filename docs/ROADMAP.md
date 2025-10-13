# Aria Roadmap

This document outlines the future direction of the Aria programming language. As an open-source project, we welcome community feedback and contributions to help shape the future of Aria. Ideas outside this roadmap are very welcome, and can be filed as GitHub issues to begin discussion.

This document is best understood alongside the [Release Policy](release_policy.md), which defines the criteria for versioning and releasing new versions of Aria.

## Very-Short-Term Goal (Aria 1.0, by December 2025)

Aria is currently at version 0.9. While we intend not to break things for its own sake, this version indicates that the language is still under development.
Up until version 1.0, we reserve the right to make breaking changes to the language and to the library. Ideally, we will only do this if the balance between the value of the change and the cost of breaking existing code is a net-positive.

After version 1.0 is released, we will only consider breaking changes on a *must* basis, i.e. we will only break existing code if there is no other realistic path to achieve a significant language goal.

Rewriting the Aria compiler or VM in Aria is a non-goal.

*   **Improve the Standard Library's Structure:** Reorganize the standard library, including:
    - move `Path` and `Regex` outside the core language;
    - consider adding a `Void::void` case and having functions default to returning that;
    - allow `func main(args: List)` as a valid entry point;
    - move `Float` exponential methods to an `extension`.

*   **Core Language Improvements:** including:
    - make backtraces available to Aria code handling exceptions;
    - improvements to cross-module variable imports;
    - float literals without sigil and decimal/complex literals;
    - ternary operator;
    - allow `;` to be a valid statement.

*   **Deliver an installable Aria package:** Allow installing `/usr/bin/aria` and running programs without compiling from source.

## Short-Term Goals (Aria 1.5, by March 2026)

*   **Improve the Core Language:** Quality of life additions to the language itself, including:
    - per-case methods in enums;
    - `char` / `byte` builtin types;
    - improved pattern matching;
    - `try_unwrap` helpers for enums;
    - `while {} else {}` support;
    - easier typecheck for function types;
    - `Either` enumeration;
    - multithreading.

*   **Enhance the Standard Library:** Add more utility modules and expand the functionality of existing ones, including:
    - server support for `aria.network`;
    - additional `aria.fs.path` helpers for filesystem manipulation;
    - better subprocess control;
    - crypto (as in, cryptography) support;
    - add more operations to `aria.numerics.Complex`.

*   **Build a REPL:** Enhance the interactive experience of Aria with a REPL. Provide better more helpful error messages.

## Medium-Term Goals (Aria 2.0, by end of 2026)

*   **Implement support for Reflection:** Allow introspecting Aria values in a more structured and uniform manner.

*   **Implement a Foreign Function Interface (FFI):** Allow Aria to call directly into C APIs without writing a glue layer.

*   **Filtered Exception Handling:** Allow pattern matching in `catch` clauses.

*   **Improvements to 3rd party library ecosystem:** Make it easy to create an Aria library (possibly with a native binding) and ship it to users.

*   **Create a Language Server Protocol (LSP) Implementation:** Provide a better development experience in code editors with features like autocompletion, go-to-definition, and inline documentation.

*   **Serializable Modules:** Allow compiling Aria modules once and storing the output for faster execution. This is not intended to allow shipping binaries that cannot be reverse-engineered, its goal is purely speed of execution for unmodified programs.

## Long-term Goals (Aria 2.x, 2027 onwards)

*   **Improve the `import` experience:** Remedy shortcomings in the way `import` works, including:
    - exclude `extension`s from being `import`ed based on importer decision;
    - exclude some symbols from being `import` based on module author decision (e.g. private impl details);
    - allow local `import`s within a function/type scope.

*   **Expand Platform Support:** Add support for Aria to run on Windows and macOS. Enable runtime platform checks.

*   **Implement a Debugger:** Develop tooling to allow interactive debugging of Aria progams.

*   **Allow GUI/TUI apps in Aria:** Add support for UI toolkits (text and graphics based).

*   **Fuzzing:** Fuzz inputs to the Aria compiler and VM and fix resulting issues.

## How to Contribute

We encourage you to get involved! Please see our [Contribution Guide](CONTRIBUTING.md) for more details on how you can help.
