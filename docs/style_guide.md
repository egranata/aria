---
title: "Aria Style Guide"
description: "Guidelines for writing clean, consistent, and maintainable Aria code."
---

# Aria Style Guide

## 1. Introduction

This document provides a set of style and coding conventions for writing Aria code. The goal is to encourage code that is readable, predictable, and consistent across the entire project. Adhering to these guidelines will make the codebase easier to understand, maintain, and extend.

This guide is based on the conventions observed in the official Aria standard library and test suite.

## 2. Naming Conventions

Consistency in naming is critical for readability. Use the following conventions for different identifiers.

### 2.1. General Naming Rules

| Identifier Type | Convention | Example(s) |
|---|---|---|
| **Modules / Files** | `snake_case.aria` | `json_parser.aria`, `http_request.aria` |
| **Variables** | `snake_case` | `val user_data = ...`, `val request_url = ...` |
| **Functions** | `snake_case` | `func calculate_hash(key) { ... }` |
| **Structs** | `PascalCase` | `struct JsonStream { ... }` |
| **Enums** | `PascalCase` | `enum TaskStatus { ... }` |
| **Enum Cases** | `PascalCase` | `case InProgress`, `case Completed` |
| **Mixins** | `PascalCase` | `mixin Iterable { ... }` |
| **Constants** | `UPPER_SNAKE_CASE` | `val SECONDS_PER_MINUTE = 60;` |

### 2.2. File Names

Aria source files should end with the `.aria` extension and be named using `snake_case`.

*Good:* `map.aria`, `file_utils.aria`
*Bad:* `Map.aria`, `file-utils.aria`

### 2.3. Type Names

Structs, enums, and mixins should be named using `PascalCase`.

```aria
struct RequestClient { ... }

enum RequestStatus { ... }

mixin Loggable { ... }
```

### 2.4. Function and Variable Names

Functions and variables should be named using `snake_case`.

```aria
func fetch_user_profile(user_id) {
    val profile_url = "https://example.com/api/users/{0}".format(user_id);
    # ...
}
```

## 3. Formatting

A consistent formatting style is essential for code that is easy to read and visually parse. Note that there is no automated formatter for Aria code, and these guidelines should not be read as strict rules, until such a formatter is made available.

### 3.1. Indentation

Use **4 spaces** for indentation. Do not use tabs.

### 3.2. Braces

The opening brace `{` should be on the same line as the declaration, separated by a space. The closing brace `}` should be on its own line, aligned with the start of the declaration.

```aria
# Good
struct MyStruct {
    func do_something() {
        if condition {
            # ...
        } else {
            # ...
        }
    }
}

# Bad: Opening brace on new line
struct MyStruct
{
    # ...
}
```

### 3.3. Spacing

- Use a single space around binary operators (`+`, `-`, `*`, `/`, `==`, etc.).
- Use a single space after commas in argument lists and list literals.
- Do not put a space between a function name and its opening parenthesis.

```aria
# Good
val result = (x + y) * z;
val items = [1, 2, 3];
func my_func(arg1, arg2) { ... }
my_func(1, 2);

# Bad
val result=(x+y)*z;
val items = [1,2,3];
func my_func (arg1, arg2) { ... }
```

### 3.4. Line Length

Keep lines under **100 characters** where possible to ensure readability.

### 3.5. Blank Lines

Use a single blank line to separate top-level function, struct, enum, or mixin definitions.

Within functions, use blank lines sparingly to group related statements into logical blocks.

## 4. Comments

Comments should explain *why* code does something, not *what* it does. The code itself should be clear enough to explain the "what".

- Use `#` for all comments.
- Place comments on the line above the code they refer to.

```aria
# Good: Explains the reason for the check.
# The remote API returns a special value for legacy users.
if user.is_legacy {
    # ...
}
```

## 5. File and Code Organization

Aria files should have a consistent structure to make them easy to navigate.

### 5.1. File Structure

Organize the contents of a `.aria` file in the following order:

1.  **License Header**: All files in the Aria standard library and test suite **must** begin with the SPDX license identifier.
    ```aria
    # SPDX-License-Identifier: Apache-2.0
    ```
2.  **File Flags** (Optional): If the file requires special handling by the VM or build system, the `flag` directive comes next.
    ```aria
    flag: no_std;
    flag: uses_dylib("aria_http");
    ```
3.  **Import Statements**: All `import` statements follow.
4.  **Helper Functions**: Free-standing helper functions that are used by the main types in the file.
5.  **Type Definitions**: The core `struct`, `enum`, or `mixin` definitions of the module. Multiple types per module may be defined, as long as they are semantically correlated. For example, the `iterator` for a type is generally defined in the same module as the type.
6.  **Extensions**: `extension` blocks that add functionality to the types. Multiple `extension` blocks may be defined in a single module, as long as they are semantically correlated. `extension`s may be used to split a type definition in multiple chunks, if the type is sufficiently large. Individual chunks should maintain their own logical grouping and possibly be ordered in dependency order.

### 5.2. Imports

- Place all `import` statements at the top of the file, after the license and flags.
- Prefer importing specific symbols with `import MyType from my.module;` to keep the local namespace clean. If multiple symbols are needed, they can be combined: `import Iterator, Iterable from aria.iterator.mixin;`. Only use `import *` as a last resort.
- Prefer to order imports alphabetically within and to group the standard library first, then your own modules and dependencies. You may use empty lines to separate groups.
- If you are importing other parts of your own widget, it is preferable to use `widget.` prefix to make it clear that it is part of your own codebase.

### 5.3. Documentation

Follow the existing documentation style and conventions as the existing standard library documentation.

## 6. Language Features Best Practices

### 6.1. Functions

#### Argument Order
Function arguments **must** be ordered as follows:
1.  Required arguments.
2.  Optional arguments (with default values).
3.  Variable arguments (`...`).

```aria
func process_data(item, retries=3, ...) {
    # ...
}
```

#### Type Checking
For public APIs and complex functions, use type hints to improve clarity and documentation.

```aria
func new_with_capacity(n: Int) {
    # ...
}
```

Use `isa` checks when type hints are not available or you need to discriminate between different possible types. Do not compare types directly. Throw `RuntimeError::UnexpectedType` only if receiving an object of an unsupported type is truly not expected by the API contract.

#### One-line functions

For functions that are one single `return` statement, you may use the one-line function syntax

```aria

# Good
func add(x,y) = x + y;

# Good
func add(x,y) = {
    return x + y;
};
```

Do not use the one-line syntax for complex or multi-line expressions

```aria

# Good
func gcd(a,b) {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

# Bad
func gcd(a,b) = a == 0 ? b : gcd(b % a, a);
```

### 6.2. Structs

- Provide a `type func new(...)` constructor to ensure instances are always created in a valid state. If multiple constructors are required, provide named constructors like `new_with_capacity` or `new_with_seed`. You may deviate from the `new` convention if a term of art exists for your constructor (e.g. the `String` to `Int` constructor is called `parse`).
- Use `alloc(This) { .field = value }` inside constructors for initialization. If multiple fields are initialized, each field should be on its own line, in the order that makes most sense for the given type.
- While the language allows for flexibility in creating fields of objects and changing their types dynamically, prefer to keep field types consistent to avoid unnecessary complexity and for documentation purposes.
- For user-facing types, implement a `prettyprint()` method to provide a readable string representation. The output should ideally be a valid representation of the object's state, like `Map(...)` or `User(...)`.

```aria
struct Map {
    type func new() {
        return Map.new_with_capacity(128);
    }

    type func new_with_capacity(n: Int) {
        return alloc(This){
            .capacity = n,
            # ...
        };
    }

    func prettyprint() {
        return "Map(...)";
    }
}
```

- If a type implements a collection, it should have the following methods, if applicable:
    - a `len()` method that returns the number of elements in the collection.
    - an `append(x)` method that adds an element to the collection, in the order that makes most sense for the given collection.
    - an `iterator()` method that returns an iterator for the collection, behaving as described in the section below.
    - an `insert(n,x)` method that inserts an element at the specified position in the collection.
    - `operator []` and `operator[]=` methods for element access and assignment.
    - a `remove(x)` method that removes the element at the specified position in the collection, or the given element from the collection, as most applicable.

`append` and `remove` may also be called `push` and `pop` respectively, if the collection is stack-like.

### 6.3. Enums

- Use enums to represent a fixed set of states or variants.
- If a case carries complex data, define a nested `struct` within the enum to represent the payload. This improves clarity and organization.

```aria
enum WebEvent {
    struct PageLoad { url: String }
    struct Click { x: Int, y: Int }

    case Load(WebEvent.PageLoad),
    case Click(WebEvent.Click),
    case KeyPress(String)
}
```

### 6.4. Operator Overloading

- When overloading operators, handle different operand types gracefully using `isa` checks.
- For unsupported types, `throw alloc(Unimplemented);`.
- For commutative binary operators (like `+` or `*`), implement the `reverse operator` to handle cases where the custom type is on the right-hand side.
- For comparison operators, prefer using the `TotalOrdering` mixin from `aria.ordering.compare`. It provides all comparison operators (`==`, `<`, `>`, etc.) based on a single `comp` method that you implement.

```aria
struct Complex {
    # ...
    operator +(rhs) {
        if (rhs isa Int) || (rhs isa Float) {
            return Complex.new(this.real + rhs, this.imag);
        } elsif rhs isa Complex {
            return Complex.new(this.real + rhs.real, this.imag + rhs.imag);
        } else {
            throw alloc(Unimplemented);
        }
    }

    reverse operator +(lhs) {
        return this._op_impl_add(lhs);
    }
}
```

- Only overload `operator()` if your object is intended to be callable like a function, e.g. a callback with state. Consider using a lambda or a free function instead.
- Avoid overloading `operator[]` for non-collection types, as this can lead to confusion. Use explicit getter methods instead. If you overload `operator[]` consider also overloading `operator[]=` and if you can't overload both, consider whether overloading either is necessary.

- Prefer upholding commonly expected invariants of your operators, e.g. `+` is commutative, `-` is not. If you have `operator u-` overloaded, ensure it behaves as a unary negation (e.g. ideally `this + u-(this) == 0` for some appropriate zero object). If your operators behave radically differently from the expectation of their symbol, consider whether overloading them is the appropriate design (e.g. `operator <<` for I/O has precedent in C++, and `operator %` can be used for string formatting in Python).

### 6.5. Iterators

The standard library follows a consistent pattern for iteration that should be adopted in user code.

- An **iterable** object (like a `List` or `Map`) must have an `iterator()` method.
- The `iterator()` method returns an **iterator** object.
- The **iterator** object must have a `next()` method.
- The `next()` method returns a `Maybe`, `Some` if there is a next item, or `None` if the iteration is complete.
- The **iterator** object may have an `iterator` method that returns itself, but this is pre-defined in the `Iterator` mixin.
- To simplify implementation, include the `Iterable` mixin in your iterable types and the `Iterator` mixin in your iterator types.

```aria
import Iterator, Iterable from aria.iterator.mixin;

struct MyCollection {
    # ...
    func iterator() {
        return MyCollectionIterator.new(this);
    }
    include Iterable
}

struct MyCollectionIterator {
    # ...
    func next() {
        if finished {
            return Maybe::None;
        }
        return Maybe::Some(next_item);
    }

    include Iterator
}
```

### 6.6. Error Handling

Follow these principles for robust error handling:

1.  **For expected absence of a value**, return a `Maybe`. This is for non-error conditions, like a key not being found in a map. The caller is expected to handle `Maybe::None`.

    ```aria
    # Good: Key might not exist, which is not an error.
    func get_from_cache(key) {
        if cache.contains(key) {
            return Maybe::Some(cache[key]);
        } else {
            return Maybe::None;
        }
    }
    ```

2.  **For expected failure of an operation**, return a `Result`. This is for expected conditions, like attempting to read a file. The caller is expected to handle `Result::Err`.

    ```aria
    # Good: File might not exist, might not be readable, ...
    func read_config() {
        if !config_path.exists() {
            return Result::Err(FileReadError.new("Configuration file not found at {0}".format(config_path)));
        }
        if !config_path.readable() {
            return Result::Err(FileReadError.new("Configuration file not readable at {0}".format(config_path)));
        }
        return Result::Ok(config_path.read());
    }
    ```

For the purposes of this sample code, of course, ignore time-of-check/time-of-use issues.

3.  **For recoverable errors**, `throw` an exception. This is for situations that are erroneous but potentially recoverable by an upstream caller, such as a transient failure. Define custom `struct`s or `enum`s for your exceptions.

    ```aria
    struct ExpiredCertificate { ... }

    func validate_certificate(path) {
        if certificate_is_expired(path) {
            throw ExpiredCertificate.new("Certificate has expired at {0}".format(path));
        }
        # ...
    }
    ```

Guidelines for exceptions:

- Include a `prettyprint` method in your exceptions, since it will be used to show any uncaught exception to the user.
- Include at least a message string as payload of your exception

```aria

# Good
struct WhatATerribleFailure {
    type func new(msg: String) {
        return alloc(This) {.msg = msg};
    }

    func prettyprint() {
        return "what a terrible failure: {0}".format(this.msg);
    }
}
```

- Exceptions should represent truly exceptional conditions, not predictable albeit suboptimal scenarios.
- If your object can throw multiple different kinds of errors, consider having an `enum` exception with cases for each possible exception.

```aria

# Good
enum TerribleFailures {
    case RemoteHostPanic(String),
    case PasswordFileNotFound(String),
    # ...

    func prettyprint() {
        match this {
            # ...
        }
    }
}

# Bad
struct RemoteHostPanic {
    type func new(msg: String) {
        return alloc(This) {.msg = msg};
    }

    func prettyprint() {
        return "remote host panic: {0}".format(this.msg);
    }
}

struct PasswordFileNotFound {
    type func new(msg: String) {
        return alloc(This) {.msg = msg};
    }

    func prettyprint() {
        return "password file not found: {0}".format(this.msg);
    }
}
```

4.  **For irrecoverable errors**, `assert` the condition. Prefer exceptions or error returns in library code as `assert` is non-recoverable for the user. In program code, `assert` liberally. In library code, `assert` sparingly and only to uphold invariants that would lead to corrupted state or operation if violated.

### 6.7 `match` statements

- If you know the type of the expression you're matching on (e.g. via a type hint), do not include `isa` checks in the match statement. Otherwise, prefer having `isa` checks to ensure type safety.
- For matching `enum` cases, prefer `isa` followed by `case` (if you need the `isa` at all).
- Avoid deep nesting of `match` statements. If you find yourself nesting `match` statements, consider refactoring your code to use a single `match` statement with more cases.
- Include an `else` case if you can't otherwise guarantee your match statement covers all possible cases.
- As Aria will always match the first case that evaluates to true, be mindful of the order of cases, prefer more specific cases first.
