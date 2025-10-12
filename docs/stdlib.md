# Aria Standard Library Reference

This document provides a comprehensive reference for the Aria standard library.

---
# `aria.core` Module Reference

This document provides a reference for the core types and built-in functionalities of the Aria language.

---

## Modules

### `aria.core.bool`

This module provides extensions to the built-in `Bool` type.

#### **Extensions**

*   **`extension Bool`**
    Extends the built-in `Bool` type.

    **Methods:**
    *   `hash()`: Returns an integer hash value for the boolean.

---

### `aria.core.box`

This module provides a simple `Box` struct, often used as a generic container for values.

#### **Structs**

*   **`Box`**
    A simple struct that can hold arbitrary fields. Often used for returning multiple values or for generic data containers.

    **Methods:**
    *   `type func new()`: Creates a new, empty `Box` instance.

---

### `aria.core.file`

This module provides functionality for file system interaction.

#### **Structs**

*   **`File`**
    Represents an open file, allowing read and write operations.

    **Methods:**
    *   `type func open(path, mode: File.OpenMode)`: Opens a file at the given `path` with the specified `mode`. `path` can be a `String` or a `Path` object. Returns a `File` object. Throws `File.IOError` on failure.
    *   `close()`: Closes the file. This is automatically called when the `File` object goes out of scope if used with `guard`.
    *   `read_all()`: Reads the entire content of the file as a `String`.
    *   `read(n: Int)`: Reads up to `n` bytes from the file and returns them as a `String`.
    *   `write(s)`: Writes a `String` to the file.
    *   `try_readln()`: Attempts to read a line from the file. Returns `Maybe::Some(line_string)` if a line is read, or `Maybe::None` if the end of the file is reached.
    *   `readln()`: Reads a line from the file. Returns an empty string if the end of the file is reached.
    *   `writeln(s)`: Writes a `String` to the file followed by a newline character.
    *   `get_position()`: Returns the current read/write position within the file as an `Int`.
    *   `set_position(offset: Int)`: Sets the read/write position within the file to `offset`.
    *   `len()`: Returns the size of the file in bytes as an `Int`.
    *   `seek(mode: File.SeekMode)`: Moves the read/write position within the file according to the specified `SeekMode`.
    *   `flush()`: Flushes any buffered writes to the underlying file system.
    *   `lines()`: Returns an iterator that yields lines from the file.

*   **`File.OpenMode`**
    A struct used to specify how a file should be opened.

    **Methods:**
    *   `type func new()`: Creates a new `OpenMode` instance with no flags set.
    *   `read()`: Sets the read flag. Returns `this` for chaining.
    *   `write()`: Sets the write flag. Returns `this` for chaining.
    *   `append()`: Sets the append flag. Returns `this` for chaining.
    *   `truncate()`: Sets the truncate flag (truncates the file to zero length if it exists). Returns `this` for chaining.
    *   `create()`: Sets the create flag (creates the file if it does not exist). Returns `this` for chaining.

#### **Enums**

*   **`File.SeekMode`**
    An enumeration specifying how to move the file pointer for `seek` operations.

    **Cases:**
    *   `Start(Int)`: Seeks to an absolute position from the beginning of the file.
    *   `Current(Int)`: Seeks relative to the current position.
    *   `End(Int)`: Seeks relative to the end of the file.

---

### `aria.core.float`

This module provides extensions to the built-in `Float` type.

#### **Extensions**

*   **`extension Float`**
    Extends the built-in `Float` type.

    **Fields:**
    *   `pi` (Float): The mathematical constant Pi (π).
    *   `e` (Float): The mathematical constant e (Euler's number).
    *   `phi` (Float): The mathematical constant Phi (φ), the golden ratio.
    *   `φ` (Float): Alias for `phi`.
    *   `π` (Float): Alias for `pi`.
    *   `inf` (Float): Represents positive infinity.
    *   `nan` (Float): Represents Not-a-Number.
    *   `epsilon` (Float): The difference between 1.0 and the next representable floating-point number.

    **Methods:**
    *   `hash()`: Returns an integer hash value for the float.
    *   `abs()`: Returns the absolute value of the float.
    *   `sqrt()`: Returns the square root of the float.
    *   `ln()`: Returns the natural logarithm (base e) of the float. Throws `Float.DomainError` if the value is non-positive.
    *   `exp()`: Returns e (Euler's number) raised to the power of the float.
    *   `pow(exponent: Int|Float)`: Returns the float raised to the power of `exponent`. Throws `Float.DomainError` for invalid operations (e.g., fractional power of a negative number).
    *   `floor()`: Returns the largest integer less than or equal to the float.
    *   `ceil()`: Returns the smallest integer greater than or equal to the float.
    *   `int()`: Returns the integer part of the float (truncates towards zero).
    *   `type func parse(s: String)`: Attempts to parse a `String` into a `Float`. Returns `Maybe::Some(Float)` on success, `Maybe::None` on failure.

---

### `aria.core.int`

This module provides extensions to the built-in `Int` type.

#### **Extensions**

*   **`extension Int`**
    Extends the built-in `Int` type.

    **Methods:**
    *   `hash()`: Returns the integer itself as its hash value.
    *   `abs()`: Returns the absolute value of the integer.
    *   `float()`: Converts the integer to a `Float`.
    *   `type func parse(s: String)`: Attempts to parse a `String` into an `Int`. It attempts to automatically detect the base used (10 by default, with support for `0x`, `0b` and `0o` prefixes). Returns `Maybe::Some(Int)` on success, `Maybe::None` on failure.
    *   `type func parse_radix(s: String, base: Int)`: Attempts to parse a `String` into an `Int` using the specified `base`. Returns `Maybe::Some(Int)` on success, `Maybe::None` on failure. Supports bases from 2 to 36, only supports a sequence of digits in the given base (no prefixes).
---

### `aria.core.list`

This module provides extensions to the built-in `List` type.

#### **Extensions**

*   **`extension List`**
    Extends the built-in `List` type.

    **Methods:**
    *   `len()`: Returns the number of elements in the list.
    *   `append(x)`: Adds an element `x` to the end of the list. Returns the modified list.
    *   `drop()`: Removes and returns the last element of the list. Throws `IndexOutOfBounds` if the list is empty.
    *   `repeat(n: Int)`: Returns a new list containing `this` list repeated `n` times.
    *   `op_mul(rhs: Int)` (Operator `*`): Returns a new list containing `this` list repeated `rhs` times.
    *   `op_rmul(lhs: Int)` (Operator `*`): Handles multiplication when the `List` is on the right-hand side of the `*` operator.
    *   `join(sep=", ")`: Returns a string representation of the list elements joined by `sep` (or ", " if sep is omitted).
    *   `contains(x)`: Returns `true` if the list contains element `x`, `false` otherwise.
    *   `type func from_function(f, n: Int)`: Creates a new list of length `n`, where each element is the result of calling function `f` with its index.
    *   `op_equals(rhs: List)` (Operator `==`): Compares this list for equality with `rhs`.
    *   `op_add(rhs: List)` (Operator `+`): Returns a new list that is the concatenation of this list and `rhs`.
    *   `quicksort_with_comparator(f)`: Sorts the list in-place using the Quicksort algorithm with a custom comparison function `f`. The function `f(a, b)` should return `true` if `a` should come before `b`.
    *   `quicksort()`: Sorts the list in-place using the Quicksort algorithm with the default `<` operator for comparison.
    *   `binary_search(target)`: Performs a binary search for `target` in a sorted list. Returns `Maybe::Some(index)` if found, `Maybe::None` otherwise.

---

### `aria.core.nothing`

This module provides the `Nothing` enum, representing the absence of a value.

#### **Enums**

*   **`Nothing`**
    An empty enumeration, used to represent the absence of a value or a type that can never be instantiated.

    **Cases:** (None)

---

### `aria.core.path`

This module provides a `Path` struct for interacting with the file system.

#### **Structs**

*   **`Path`**
    Represents a file system path, providing methods for path manipulation and file system operations.

    **Methods:**
    *   `type func new(s: String)`: Creates a new `Path` object from a string representation.
    *   `type func new_with_current_directory()`: Returns a `Path` object representing the current working directory.
    *   `type func new_with_environment_variable(var)`: Returns a `Path` object from the value of an environment variable `var`.
    *   `append(rhs: String|Path)`: Appends a component (`String` or `Path`) to the end of this path. Returns the modified path.
    *   `op_div(rhs: String|Path)` (Operator `/`): Returns a new `Path` object by joining this path with `rhs`.
    *   `parent()`: Returns a new `Path` object representing the parent directory of this path.
    *   `read()`: Reads the entire content of the file at this path as a `String`. Throws `File.IOError` on failure.
    *   `write(text)`: Writes a `String` `text` to the file at this path, overwriting existing content. Throws `File.IOError` on failure.
    *   `accessed()`: Returns the last access time of the file/directory at this path as an `Instant`. Returns `Maybe::None` if not available.
    *   `created()`: Returns the creation time of the file/directory at this path as an `Instant`. Returns `Maybe::None` if not available.
    *   `modified()`: Returns the last modification time of the file/directory at this path as an `Instant`. Returns `Maybe::None` if not available.
    *   `copy_to(other: Path)`: Copies the file at this path to `other`. Returns `true` on success, `false` on failure.
    *   `is_absolute()`: Returns `true` if the path is absolute, `false` otherwise.
    *   `exists()`: Returns `true` if the path exists on the file system, `false` otherwise.
    *   `is_directory()`: Returns `true` if the path points to a directory, `false` otherwise.
    *   `is_file()`: Returns `true` if the path points to a regular file, `false` otherwise.
    *   `is_symlink()`: Returns `true` if the path points to a symbolic link, `false` otherwise.
    *   `new_canonical()`: Returns a new `Path` object representing the canonical, absolute path (resolving symlinks). Returns `Maybe::None` if the canonical path cannot be resolved.
    *   `size()`: Returns the size of the file at this path in bytes as an `Int`. Returns `Maybe::None` if not a file or path does not exist.
    *   `get_filename()`: Returns the final component of the path (the file or directory name) as a `String`. Returns `Maybe::None` if the path has no filename component.
    *   `get_extension()`: Returns the extension of the file at this path as a `String`. Returns `Maybe::None` if the path has no extension.
    *   `entries()`: Returns a `List` of `Path` objects representing the entries (files and subdirectories) within the directory pointed to by this path.
    *   `mkdir()`: Creates a new directory at this path. Returns `true` on success, `false` on failure.
    *   `rmdir()`: Removes an empty directory at this path. Returns `true` on success, `false` on failure.
    *   `erase()`: Removes the file at this path. Returns `true` on success, `false` on failure.
    *   `common_ancestor(p: Path)`: Returns the common ancestor path between this path and `p`, as a `Maybe`.

---

### `aria.core.result`

This module defines a dynamic success/error carrier and bridges with `Maybe` and exceptions.

#### **Enums**

*   **`Result`**
    Represents the outcome of an operation, which can be either a success with a value or an error with a message.

    **Cases:**
    *   `Ok(Any)`: Contains a successful result of any type.
    *   `Err(Any)`: Contains an error message.

    **Methods:**
    *   `is_Ok()`: Returns `true` if the `Result` is `Ok`, `false` otherwise.
    *   `is_Err()`: Returns `true` if the `Result` is `Err`, `false` otherwise.
    *   `unwrap_Ok()`: Returns the value contained within `Ok`.
    *   `unwrap_Err()`: Returns the value contained within `Err`.
    *   `or_throw()`: If `Ok(v)`, returns `v`. If `Err(e)`, throws `e`.
    *   `unwrap_or(default_value)`: Returns the value contained within `Ok`, or `default_value` if it is `Err`.
    *   `apply(f)`: Result `f(v)` if the `Result` is `Ok(v)`. If `Result` is `Err`, returns the `Err` unchanged.
    *   `type func new_with_maybe(m: Maybe)`: Returns `Ok(v)` if `m` is `Some(v)`, returns `Err(Unit.new())` if `m` is `None`.
    *   `type func new_with_try(f)`: Executes `f()`. Returns `Ok(result)` if `f` completes. Returns `Err(e)` if `f` throws `e`.

#### **Extensions**

*   **`extension Maybe`**
    Extends the built-in `Maybe` type.

    **Methods:**
    *   `new_with_result(r: Result):`: Returns `Maybe::Some(v)` if `r` is `Ok(v)`, returns `Maybe::None` if `r` is `Err`.

#### **Functions**
*   `ok(v)`: Shorthand constructor. Returns `Result::Ok(v)`.
*   `err(e)`: Shorthand constructor. Returns `Result::Err(e)`.

---

### `aria.core.string`

This module provides extensions to the built-in `String` type.

#### **Extensions**

*   **`extension String`**
    Extends the built-in `String` type.

    **Methods:**
    *   `repeat(n: Int)`: Returns a new string containing `this` string repeated `n` times.
    *   `op_mul(rhs: Int)` (Operator `*`): Returns a new string containing `this` string repeated `rhs` times.
    *   `op_rmul(lhs: Int)` (Operator `*`): Handles multiplication when the `String` is on the right-hand side of the `*` operator.
    *   `trim_head()`: Returns a new string with leading whitespace removed.
    *   `trim_tail()`: Returns a new string with trailing whitespace removed.
    *   `format(...)`: Formats the string using positional arguments. Placeholders like `{0}` are replaced by corresponding arguments. Supports `{{` and `}}` for literal braces.
    *   `substring(from: Int, to: Int)`: Returns a new string that is a substring of `this` string, starting at `from` (inclusive) and ending at `to` (inclusive).
    *   `hash()`: Returns an integer hash value for the string.
    *   `join(iter)`: Joins elements from an iterable `iter` into a single string, with `this` string as the separator.
    *   `len()`: Returns the length of the string in characters.
    *   `has_prefix(prefix: String)`: Returns `true` if the string starts with `prefix`, `false` otherwise.
    *   `has_suffix(suffix: String)`: Returns `true` if the string ends with `suffix`, `false` otherwise.
    *   `replace(current: String, wanted: String)`: Returns a new string with all occurrences of `current` replaced by `wanted`.
    *   `split(marker: String)`: Splits the string by `marker` and returns a `List` of substrings.
    *   `chars()`: Returns a `List` of single-character strings representing the characters in the string.
    *   `bytes()`: Returns a `List` of integers representing the UTF-8 byte values of the string.
    *   `type func new_with_bytes(bytes: List)`: Creates a new `String` from a `List` of byte integers. Throws `String.EncodingError` if the bytes are not valid UTF-8.
    *   `encoding()`: Returns the numeric UTF-8 encoding of the first character of the string as an `Int`. (Intended for single-character strings).
    *   `uppercase()`: Returns a new string with all characters converted to uppercase.
    *   `lowercase()`: Returns a new string with all characters converted to lowercase.
    *   `contains(substring: String)`: Returns `true` if the string contains the `substring`, `false` otherwise.

---

### `Maybe` (Built-in Enum)

`Maybe` is a built-in enumeration that represents a value that may or may not be present. It is similar to `Optional` in other languages.

#### **Enums**

*   **`Maybe`**
    An enumeration that can either contain a value (`Some`) or indicate the absence of a value (`None`).

    **Cases:**
    *   `Some(Any)`: Contains a value of any type.
    *   `None`: Represents the absence of a value.

    **Methods:**
    *   `is_Some()`: Returns `true` if the `Maybe` contains a value (`Some`), `false` otherwise.
    *   `is_None()`: Returns `true` if the `Maybe` does not contain a value (`None`), `false` otherwise.
    *   `unwrap_Some()`: Returns the value contained within `Some`. Throws `EnumWithoutPayload` if called on `None`.
    *   `unwrap_or(default_value)`: Returns the value contained within `Some`, or `default_value` if it is `None`.
    *   `apply(f)`: If `Maybe` is `Some(value)`, applies the function `f` to `value` and returns the result. If `Maybe` is `None`, returns `None`.

---

### `Unit` (Built-in Enum)

`Unit` is a built-in enumeration that represents the absence of any meaningful value. It is similar to `void` in C-like languages or `()` in Rust.

#### **Enums**

*   **`Unit`**
    An enumeration with a single case, used to indicate that a function returns no meaningful value, or as a placeholder where a type is required but no data is conveyed.

    **Cases:** `unit`: Represents the unit value

---

# `aria.date` Module Reference

This document provides a reference for the `aria.date` module, which contains utilities for handling dates and times.

---

## Modules

### `aria.date.instant`

This module provides a struct for representing a specific moment in time, similar to a timestamp, but with calendar and time components.

#### **Structs**

*   **`Instant`**
    An object that represents a single point in time, broken down into calendar and clock elements. It can be created from a Unix timestamp and can account for timezone offsets.

    **Fields:**
    *   `year` (Int): The year (e.g., 2025).
    *   `month` (Int): The month of the year (1-12).
    *   `day` (Int): The day of the month (1-31).
    *   `hour` (Int): The hour of the day (0-23).
    *   `minute` (Int): The minute of the hour (0-59).
    *   `second` (Int): The second of the minute (0-59).
    *   `millisecond` (Int): The millisecond of the second (0-999).
    *   `unix_ts` (Int): The original Unix timestamp in milliseconds that this `Instant` was created from.
    *   `offset_ms` (Int): The timezone offset from UTC in milliseconds.

    **Methods:**
    *   `type func now()`: Creates a new `Instant` object that represent the current time, in the local timezone.
    *   `type func new_with_utc_timestamp(timestamp_ms)`: Creates a new `Instant` object from a provided Unix timestamp (in milliseconds), assuming UTC.
    *   `type func new_with_timestamp_and_offset(timestamp_ms, offset_minutes)`: Creates a new `Instant` object from a provided Unix timestamp (in milliseconds), adjusted for a given timezone offset (in minutes).
    *   `type func new_with_local_timestamp(timestamp_ms)`: Creates a new `Instant` object from a provided Unix timestamp (in milliseconds), assuming the timestamp is in the local timezone.
    *   `instance func with_timezone_offset(offset_minutes)`: Changes the current `Instant` object with the timezone offset adjusted to the given offset in minutes.
    *   `instance func prettyprint()`: Returns a formatted string representation of the `Instant`.

---

### `aria.date.timezone`

This module provides a function to collect timezone information.

#### **Functions**
*   `timezone_info()`: Returns a `List` containing the timezone offset in minutes (Int) and the timezone name (String).

---
# `aria.iterator` Module Reference

This document provides a reference for the `aria.iterator` module, which contains core interfaces and utilities for working with iterators and iterable collections.

---

## Modules

### `aria.iterator.mixin`

This module defines the fundamental `Iterator` and `Iterable` mixins, which enable collections to be traversed and transformed.

#### **Mixins**

*   **`Iterator`**
    A mixin that defines the contract for an object that can be iterated over. It provides common functional methods for transforming the iteration stream.

    **Requirements:**
    *   The struct including this mixin **must** implement an instance method `next()` that returns a `Box` object with two fields: `.done` (a `Bool` indicating if the iteration is complete) and `.value` (the current item, present only if `.done` is `false`).

    **Methods Offered:**
    *   `map(f)`: Returns a new iterator that applies the function `f` to each item yielded by this iterator.
    *   `where(f)`: Returns a new iterator that yields only the items for which the predicate function `f` returns `true`.
    *   `reduce(f, initial)`: Applies a function `f` against an accumulator and each item in the iterator (from left to right) to reduce it to a single value. `initial` is the starting value of the accumulator.
    *   `to_list()`: Consumes the iterator and returns a `List` containing all its items.
    *   `flatten_results()`: Consumes the iterator. If any items are `Result::Err(x)` returns `err(x)`. Otherwise, it returns `ok(List)` of all the unwrapped `Result::Ok(value)` items. Non-`Result` values are appended to the result list verbatim.
    *   `all(f)`: Returns `true` if the predicate function `f` returns `true` for all items in the iterator, `false` otherwise. This method short-circuits, i.e. it stops consuming the iterator as soon as the outcome is determined.
    *   `any(f)`: Returns `true` if the predicate function `f` returns `true` for at least one item in the iterator, `false` otherwise. This method short-circuits, i.e. it stops consuming the iterator as soon as the outcome is determined.
    *   `find(f)`: Returns `Maybe::Some(value)` for the first item for which the predicate function `f` returns `true`, or `Maybe::None` if no such item is found.
    *   `position(f)`: Returns `Maybe::Some(index)` for the first item for which the predicate function `f` returns `true`, or `Maybe::None` if no such item is found.
    *   `sum()`: Consumes the iterator and returns the sum of all its items. Assumes items support the `+` operator.
    *   `product()`: Consumes the iterator and returns the product of all its items. Assumes items support the `*` operator.
    *   `max()`: Returns `Maybe::Some(value)` with the maximum value in the iterator, or `Maybe::None` if the iterator is empty. Assumes items support the `>` operator.
    *   `min()`: Returns `Maybe::Some(value)` with the minimum value in the iterator, or `Maybe::None` if the iterator is empty. Assumes items support the `<` operator.
    *   `count()`: Consumes the iterator and returns the total number of items.
    *   `first()`: Returns `Maybe::Some(value)` with the first item of the iterator, or `Maybe::None` if the iterator is empty.
    *   `last()`: Consumes the iterator and returns `Maybe::Some(value)` with the last item, or `Maybe::None` if the iterator was empty.
    *   `nth(n: Int)`: Consumes the iterator up to the nth item and returns `Maybe::Some(value)`. Returns `Maybe::None` if `n` is negative or the iterator has fewer than `n+1` items.
    *   `iterator()`: Returns the iterator itself, allowing an `Iterator` to be used where an iterable value is expected.

*   **`Iterable`**
    A mixin that defines the contract for an object that can produce an `Iterator`. It provides convenience methods that delegate to the iterator produced by the `iterator()` method.

    **Requirements:**
    *   The struct including this mixin **must** implement an instance method `iterator()` that returns an `Iterator` object.

    **Methods Offered:**
    *   `map(f)`: Returns a new iterator that applies the function `f` to each item yielded by this iterable's iterator.
    *   `where(f)`: Returns a new iterator that yields only the items for which the predicate function `f` returns `true`.
    *   `reduce(f, initial)`: Applies a function `f` against an accumulator and each item in the iterable (from left to right) to reduce it to a single value. `initial` is the starting value of the accumulator.
    *   `to_list()`: Returns a `List` containing all items from this iterable's iterator.
    *   `all(f)`: Returns `true` if the predicate function `f` returns `true` for all items, `false` otherwise.
    *   `any(f)`: Returns `true` if the predicate function `f` returns `true` for at least one item, `false` otherwise.
    *   `find(f)`: Returns `Maybe::Some(value)` for the first item for which the predicate function `f` returns `true`, or `Maybe::None` if no such item is found.
    *   `position(f)`: Returns `Maybe::Some(index)` for the first item for which the predicate function `f` returns `true`, or `Maybe::None` if no such item is found.
    *   `sum()`: Returns the sum of all items. Assumes items support the `+` operator.
    *   `product()`: Returns the product of all items. Assumes items support the `*` operator.
    *   `max()`: Returns `Maybe::Some(value)` with the maximum value, or `Maybe::None` if the iterable is empty. Assumes items support the `>` operator.
    *   `min()`: Returns `Maybe::Some(value)` with the minimum value, or `Maybe::None` if the iterable is empty. Assumes items support the `<` operator.
    *   `count()`: Returns the total number of items.
    *   `first()`: Returns `Maybe::Some(value)` with the first item, or `Maybe::None` if the iterable is empty.
    *   `last()`: Returns `Maybe::Some(value)` with the last item, or `Maybe::None` if the iterable is empty.
    *   `nth(n: Int)`: Returns `Maybe::Some(value)` for the nth item. Returns `Maybe::None` if `n` is out of bounds.

---

### `aria.iterator.enumerate`

This module provides an extension to `Iterable` that allows iterating with an index.

#### **Extensions**

*   **`extension Iterable`**
    Extends any `Iterable` object with an `enumerate` method.

    **Methods:**
    *   `enumerate()`: Returns a new iterator that yields `Box` objects with `.index` (the 0-based index) and `.value` (the item) for each item in the iterable.

---

### `aria.iterator.zip`

This module provides an extension to `Iterable` that allows combining two iterables into one.

#### **Extensions**

*   **`extension Iterable`**
    Extends any `Iterable` object with a `zip` method.

    **Methods:**
    *   `zip(other)`: Returns a new iterator that yields `Box` objects with `.first` (from this iterable) and `.second` (from the `other` iterable) for each corresponding pair of items. The iteration stops when either iterable is exhausted.

---
# `aria.json` Module Reference

This document provides a reference for the `aria.json` module, which contains functionality for parsing and generating JSON data.

---

## Modules

### `aria.json.value`

This module defines the core `JsonValue` enum, which represents any JSON data type.

#### **Structs**

*   **`JsonNull`**
    A placeholder struct representing the JSON `null` value.

#### **Enums**

*   **`JsonValue`**
    An enumeration that can hold any valid JSON data type.

    **Cases:**
    *   `Object(Map)`: Represents a JSON object, mapping string keys to `JsonValue`s.
    *   `Array(List)`: Represents a JSON array, a list of `JsonValue`s.
    *   `String(String)`: Represents a JSON string.
    *   `Number(Float)`: Represents a JSON number.
    *   `Boolean(Bool)`: Represents a JSON boolean (`true` or `false`).
    *   `Null(JsonNull)`: Represents a JSON `null` value.

    **Methods:**
    *   `flatten()`: Recursively converts a `JsonValue` (and its nested `Object`s and `Array`s) into native Aria `Map`s, `List`s, `String`s, `Float`s, `Bool`s, and `JsonNull`s. This is useful for working with parsed JSON data using standard Aria types.
    *   `type func new_from_value(x)`: Converts a native Aria value (`String`, `Int`, `Float`, `Bool`, `List`, `Map`, `JsonNull`) into its corresponding `JsonValue` enum case. For `List`s and `Map`s, the conversion is recursive. If `x` is a custom type that has a `to_json_value()` method, that method will be called to perform the conversion. Throws `JsonConvertError` if the type cannot be converted.

---

### `aria.json.parser`

This module provides functionality for parsing JSON strings into `JsonValue` objects.

#### **Extensions**

*   **`extension JsonValue`**
    Extends the `JsonValue` enum with parsing capabilities.

    **Methods:**
    *   `type func parse(s)`: Parses a JSON formatted `String` `s` and returns a `JsonValue` representation of the data. Throws `JsonParseError` if the string is not valid JSON.

---

### `aria.json.writer`

This module provides functionality for serializing `JsonValue` objects into JSON strings.

#### **Extensions**

*   **`extension JsonValue`**
    Extends the `JsonValue` enum with serialization capabilities.

    **Methods:**
    *   `to_json_string()`: Converts the `JsonValue` object into its JSON string representation. This method handles the recursive serialization of nested objects and arrays.

---
# `aria.network` Module Reference

This document provides a reference for the `aria.network` module, which contains functionality for making HTTP requests.

---

## Modules

### `aria.network.request`

This module provides the `Request` struct for building and sending HTTP requests.

#### **Structs**

*   **`Request`**
    A builder-style struct used to configure and send HTTP requests.

    **Fields:**
    *   `url` (String): The URL for the request.
    *   `headers` (Map): A map of header names to header values for the request.
    *   `timeout` (Float): The timeout for the request in seconds.

    **Methods:**
    *   `type func new(url: String)`: Creates a new `Request` instance for the given `url`, with an empty headers map and a default timeout.
    *   `get()`: Sends an HTTP GET request using the configured `Request` object. Returns a `Request.Response` object.
    *   `post(data: String)`: Sends an HTTP POST request with the given `data` string as the body. Returns a `Request.Response` object.
    *   `post_as_json(data)`: Sends an HTTP POST request with the given `data` (which can be any native Aria type convertible to JSON) as a JSON-formatted body. Automatically sets the `Content-Type` header to `application/json`. Returns a `Request.Response` object.

*   **`Request.Response`**
    An inner struct within `Request` that represents the response received from an HTTP request.

    **Fields:**
    *   `status_code` (Int): The HTTP status code of the response (e.g., 200, 404).
    *   `headers` (Map): A map of header names to header values from the response.
    *   `content` (String): The body of the response as a string.

---
# `aria.numerics` Module Reference

This document provides a reference for the `aria.numerics` module, which contains advanced numerical types and functions.

---

## Modules

### `aria.numerics.complex`

This module provides a `Complex` number type.

#### **Structs**

*   **`Complex`**
    Represents a complex number with real and imaginary parts.

    **Fields:**
    *   `real` (Float): The real component of the complex number.
    *   `imag` (Float): The imaginary component of the complex number.

    **Methods:**
    *   `type func new(r, i)`: Creates a new `Complex` number with real part `r` and imaginary part `i`.
    *   `type func zero()`: Returns a new `Complex` number representing zero (0 + 0i).
    *   `conj()`: Returns the complex conjugate of the number.
    *   `reciprocal()`: Returns the reciprocal (1/z) of the complex number.
    *   `op_add(rhs)` (Operator `+`): Adds this complex number to `rhs`. `rhs` can be an `Int`, `Float`, or `Complex`.
    *   `op_radd(lhs)` (Operator `+`): Handles addition when the `Complex` number is on the right-hand side of the `+` operator.
    *   `op_mul(rhs)` (Operator `*`): Multiplies this complex number by `rhs`. `rhs` can be an `Int`, `Float`, or `Complex`.
    *   `op_rmul(lhs)` (Operator `*`): Handles multiplication when the `Complex` number is on the right-hand side of the `*` operator.
    *   `op_div(rhs)` (Operator `/`): Divides this complex number by `rhs`. `rhs` can be an `Int`, `Float`, or `Complex`.
    *   `op_rdiv(lhs)` (Operator `/`): Handles division when the `Complex` number is on the right-hand side of the `/` operator.
    *   `op_sub(rhs)` (Operator `-`): Subtracts `rhs` from this complex number. `rhs` can be an `Int`, `Float`, or `Complex`.
    *   `op_rsub(lhs)` (Operator `-`): Handles subtraction when the `Complex` number is on the right-hand side of the `-` operator.
    *   `op_equals(rhs)` (Operator `==`): Compares this complex number for equality with `rhs`. `rhs` can be an `Int`, `Float`, or `Complex`.

---

### `aria.numerics.decimal`

This module provides a `Decimal` number type for arbitrary-precision decimal arithmetic.

#### **Structs**

*   **`Decimal`**
    Represents a decimal number with a specific value and scale, designed for precise financial or scientific calculations where floating-point inaccuracies are unacceptable.

    **Fields:**
    *   `value` (Int): The unscaled integer value of the decimal number.
    *   `scale` (Int): The number of digits after the decimal point.

    **Methods:**
    *   `type func new(v)`: Creates a new `Decimal` from an `Int` or `Float` value.
    *   `type func new_with_parts(v: Int, s: Int)`: Creates a new `Decimal` from an integer `value` and a `scale`.
    *   `type func parse(s: String)`: Creates a new `Decimal` from a `String`. Returns a `Maybe` value.
    *   `op_add(other)` (Operator `+`): Adds this decimal number to `other`. `other` can be an `Int`, `Float`, or `Decimal`.
    *   `op_radd(lhs)` (Operator `+`): Handles addition when the `Decimal` number is on the right-hand side of the `+` operator.
    *   `op_sub(other)` (Operator `-`): Subtracts `other` from this decimal number. `other` can be an `Int`, `Float`, or `Decimal`.
    *   `op_rsub(lhs)` (Operator `-`): Handles subtraction when the `Decimal` number is on the right-hand side of the `-` operator.
    *   `op_mul(other)` (Operator `*`): Multiplies this decimal number by `other`. `other` can be an `Int`, `Float`, or `Decimal`.
    *   `op_rmul(lhs)` (Operator `*`): Handles multiplication when the `Decimal` number is on the right-hand side of the `*` operator.
    *   `op_div(other)` (Operator `/`): Divides this decimal number by `other`. `other` can be an `Int`, `Float`, or `Decimal`.
    *   `op_rdiv(lhs)` (Operator `/`): Handles division when the `Decimal` number is on the right-hand side of the `/` operator.
    *   `comp(other)`: Compares this decimal number to `other` and returns a `CompareResult` (`lt`, `eq`, `gt`). `other` can be an `Int`, `Float`, or `Decimal`.
    *   *(Included from `TotalOrdering`)* `op_equals(rhs)` (Operator `==`)
    *   *(Included from `TotalOrdering`)* `op_lt(rhs)` (Operator `<`)
    *   *(Included from `TotalOrdering`)* `op_gt(rhs)` (Operator `>`)
    *   *(Included from `TotalOrdering`)* `op_lteq(rhs)` (Operator `<=`)
    *   *(Included from `TotalOrdering`)* `op_gteq(rhs)` (Operator `>=`)

---

### `aria.numerics.matrix`

This module provides a `Matrix` type for linear algebra operations.

#### **Structs**

*   **`Matrix`**
    Represents a mathematical matrix, supporting common matrix operations.

    **Fields:**
    *   `rows` (Int): The number of rows in the matrix.
    *   `cols` (Int): The number of columns in the matrix.
    *   `data` (Map): Internal storage for matrix elements, mapping `MatrixIndex` to values.

    **Methods:**
    *   `type func new(rows: Int, cols: Int)`: Creates a new `Matrix` with the specified number of rows and columns, initialized with `0.0f`.
    *   `get(row: Int, col: Int)`: Retrieves the value at the specified `row` and `col`. Throws `Matrix.DimensionMismatch` if indices are out of bounds.
    *   `set(row: Int, col: Int, value)`: Sets the `value` at the specified `row` and `col`. Throws `Matrix.DimensionMismatch` if indices are out of bounds.
    *   `op_add(other: Matrix)` (Operator `+`): Adds this matrix to `other`. Throws `Matrix.DimensionMismatch` if dimensions do not match.
    *   `op_sub(other: Matrix)` (Operator `-`): Subtracts `other` from this matrix. Throws `Matrix.DimensionMismatch` if dimensions do not match.
    *   `op_mul(other: Matrix)` (Operator `*`): Multiplies this matrix by `other`. Throws `Matrix.DimensionMismatch` if dimensions do not match for multiplication.
    *   `transpose()`: Returns a new `Matrix` that is the transpose of this matrix.
    *   `determinant()`: Calculates the determinant of the matrix. Throws `Matrix.DimensionMismatch` if the matrix is not square.
    *   `op_equals(other: Matrix)` (Operator `==`): Compares this matrix for equality with `other`.

---

### `aria.numerics.trig`

This module provides trigonometric functions as extensions to the `Float` type.

#### **Extensions**

*   **`extension Float`**
    Extends the built-in `Float` type with trigonometric and inverse trigonometric functions.

    **Methods:**
    *   `sin()`: Returns the sine of the float value (in radians).
    *   `cos()`: Returns the cosine of the float value (in radians).
    *   `tan()`: Returns the tangent of the float value (in radians).
    *   `arcsin()`: Returns the arcsine (inverse sine) of the float value (in radians). Throws `Float.DomainError` if the input is outside `[-1.0, 1.0]`.
    *   `arccos()`: Returns the arccosine (inverse cosine) of the float value (in radians). Throws `Float.DomainError` if the input is outside `[-1.0, 1.0]`.
    *   `arctan()`: Returns the arctangent (inverse tangent) of the float value (in radians).

---
# `aria.ordering` Module Reference

This document provides a reference for the `aria.ordering` module, which contains utilities and mixins for comparing and ordering values.

---

## Modules

### `aria.ordering.compare`

This module defines the result of a comparison operation and a mixin for implementing total ordering.

#### **Enums**

*   **`CompareResult`**
    An enumeration representing the outcome of a comparison between two values.

    **Cases:**
    *   `lt`: The first value is less than the second.
    *   `eq`: The first value is equal to the second.
    *   `gt`: The first value is greater than the second.

#### **Mixins**

*   **`TotalOrdering`**
    A mixin that provides standard comparison operators (`==`, `<`, `>`, `<=`, `>=`) for a struct, given that the struct implements a core comparison method.

    **Requirements:**
    *   The struct including this mixin **must** implement an instance method `comp(other)` that returns a `CompareResult` enum value (`lt`, `eq`, or `gt`) indicating the relationship between `this` object and `other`.

    **Methods Offered:**
    *   `op_equals(rhs)` (Operator `==`): Returns `true` if `this` is equal to `rhs`.
    *   `op_lt(rhs)` (Operator `<`): Returns `true` if `this` is less than `rhs`.
    *   `op_gt(rhs)` (Operator `>`): Returns `true` if `this` is greater than `rhs`.
    *   `op_lteq(rhs)` (Operator `<=`): Returns `true` if `this` is less than or equal to `rhs`.
    *   `op_gteq(rhs)` (Operator `>=`): Returns `true` if `this` is greater than or equal to `rhs`.

---

### `aria.ordering.utils`

This module provides utility functions for finding minimum, maximum, and min-max values in a list.

#### **Functions**

*   `min(l: List)`: Returns the minimum value in the provided list `l` using the default `<` operator. Throws an error if the list is empty.
*   `min_with_comparator(l: List, cmp)`: Returns the minimum value in the provided list `l` using a custom comparator function `cmp`. The `cmp` function should take two arguments and return a `CompareResult`.
*   `max(l: List)`: Returns the maximum value in the provided list `l` using the default `>` operator. Throws an error if the list is empty.
*   `max_with_comparator(l: List, cmp)`: Returns the maximum value in the provided list `l` using a custom comparator function `cmp`. The `cmp` function should take two arguments and return a `CompareResult`.
*   `min_max(l: List)`: Returns a `Box` object with `.min` and `.max` fields, containing the minimum and maximum values in the provided list `l` using default comparison operators. Throws an error if the list is empty.
*   `min_max_with_comparator(l: List, cmp)`: Returns a `Box` object with `.min` and `.max` fields, containing the minimum and maximum values in the provided list `l` using a custom comparator function `cmp`. The `cmp` function should take two arguments and return a `CompareResult`.

---

# `aria.system` Module Reference

This document provides a reference for the `aria.system` module, which contains utilities for interacting with the operating system.

---

## Modules

### `aria.system.platform` Module Reference

This module provides functionality for representing and identifying the operating system platform at runtime.

#### Enums

*   **`Platform`**
An enumeration representing the supported operating system platforms.

    **Cases:**
    *   `Linux(Platform.LinuxPlatform)`: Represents a Linux platform, with kernel version information.
    *   `macOS(Platform.macOSPlatform)`: Represents a macOS platform, with OS build information.
    *   `Unknown`: Represents an unrecognized or unsupported platform.

    **Methods:**
    *   `prettyprint()`: Returns a formatted string describing the platform.  
    *   name()`: Returns the platform name as a `String`.  

---

## Structs

*   **`Platform.LinuxPlatform`**
Represents Linux-specific platform data.

    **Fields:**
    *   `kernel_version` (`String`): The Linux kernel version string.

    **Methods:**
    *   `type func new(kernel_version: String)`: Creates a new `LinuxPlatform` instance with the specified kernel version.
    *   `name()`: Returns `"Linux"`.

---

*   **`Platform.macOSPlatform`**
Represents macOS-specific platform data.

    **Fields:**
    *   `os_build` (`String`): The macOS build identifier.

    **Methods:**
    *   `type func new(os_build: String)`: Creates a new `macOSPlatform` instance with the specified OS build identifier.
    *   `name()`: Returns `"macOS"`.
---

### `aria.system.coloring`

This module provides utilities for coloring terminal output using ANSI escape codes.

#### **Enums**

*   **`Color`**
    An enumeration representing standard terminal colors.

    **Cases:**
    *   `Black`, `Red`, `Green`, `Yellow`, `Blue`, `Magenta`, `Cyan`, `White`
    *   `BrightBlack`, `BrightRed`, `BrightGreen`, `BrightYellow`, `BrightBlue`, `BrightMagenta`, `BrightCyan`, `BrightWhite`
    *   `RGB(Color.RGB)`: Represents a 24-bit RGB color.

#### **Structs**

*   **`Color.RGB`**
    A struct to represent a 24-bit RGB color value.

    **Fields:**
    *   `red` (Int): The red component (0-255).
    *   `green` (Int): The green component (0-255).
    *   `blue` (Int): The blue component (0-255).

    **Methods:**
    *   `type func new(red: Int, green: Int, blue: Int)`: Creates a new `RGB` color. Values are clamped to the 0-255 range.
    *   `type func new_with_hex_string(s: String)`: Creates a new `RGB` color from a hex string (e.g., `#RRGGBB` or `RRGGBB`). Returns `Maybe::Some(Color.RGB)` on success, `Maybe::None` on failure.

*   **`ColorScheme`**
    A struct for defining a combination of text styles (foreground color, background color, bold).

    **Methods:**
    *   `type func new()`: Creates a new, empty `ColorScheme`.
    *   `reset()`: Resets the color scheme to its default (empty) state.
    *   `with_background_color(c: Color)`: Sets the background color for the scheme. Returns `this` for chaining.
    *   `with_foreground_color(c: Color)`: Sets the foreground color for the scheme. Returns `this` for chaining.
    *   `with_bold(b: Bool)`: Enables or disables the bold attribute for the scheme. Returns `this` for chaining.
    *   `apply(s: String)`: Applies the defined color scheme to a given string `s`, returning a new string with the appropriate ANSI escape codes.

#### **Extensions**

*   **`extension String`**
    Extends the built-in `String` type with methods for easily applying colors and styles.

    **Methods:**
    *   `with_background_color(c: Color)`: Applies the specified `Color` to the string's background.
    *   `with_foreground_color(c: Color)`: Applies the specified `Color` to the string's foreground.
    *   `with_bold()`: Applies bold styling to the string.
    *   `with_style(s: ColorScheme)`: Applies a full `ColorScheme` to the string.
    *   `black()`, `red()`, `green()`, `yellow()`, `blue()`, `magenta()`, `cyan()`, `white()`: Shorthand methods for setting the foreground color.
    *   `black_bg()`, `red_bg()`, `green_bg()`, `yellow_bg()`, `blue_bg()`, `magenta_bg()`, `cyan_bg()`, `white_bg()`: Shorthand methods for setting the background color.
    *   `bright_black()`, `bright_red()`, etc.: Shorthand methods for setting the bright foreground color.
    *   `bright_black_bg()`, `bright_red_bg()`, etc.: Shorthand methods for setting the bright background color.
---

# `aria.range` Module Reference

This document provides a reference for the `aria.range` module, which contains utilities for creating and manipulating numeric ranges.

---

## Modules

### `aria.range.int_extension`

This module extends the built-in `Int` type with convenient methods for creating ranges.

#### **Extensions**

*   **`extension Int`**
    Extends the built-in `Int` type with methods to easily create `Range` objects.

    **Methods:**
    *   `to(n: Int)`: Creates a range from `this` integer up to (but not including) `n`. For example, `1.to(5)` creates a range `[1, 2, 3, 4]`.
    *   `through(n: Int)`: Creates a range from `this` integer up to and including `n`. For example, `1.through(5)` creates a range `[1, 2, 3, 4, 5]`.

---

### `aria.range.range`

This module provides the core `Range` types and their associated functionality.

#### **Structs**

*   **`RangeImpl`**
    Represents a concrete numeric range (e.g., `[from, to)`). This is the object returned by `Range.from(...).to(...)` or `Range.from(...).through(...)`.

    **Methods:**
    *   `step(n)`: Returns an iterator that iterates through the range with the specified step `n`. Throws `InvalidRangeError` if `n` is zero.
    *   `iterator()`: Returns an iterator that iterates through the range with a step of `1`.
    *   `descending()`: Returns an iterator that iterates through the range in reverse order with a step of `-1`.
    *   `contains(x)`: Returns `true` if `x` is within the range, `false` otherwise.
    *   `length()`: Returns the number of elements in the range.
    *   `union(other)`: Returns a new `RangeImpl` representing the union of this range and `other`.
    *   `intersection(other)`: Returns a new `RangeImpl` representing the intersection of this range and `other`.
    *   `prettyprint()`: Returns a string representation of the range.

*   **`RangeFrom`**
    An intermediate object used in the fluent API for creating ranges (e.g., `Range.from(X)` returns a `RangeFrom` object).

    **Methods:**
    *   `type func new(n)`: Creates a new `RangeFrom` object starting from `n`.
    *   `to(n)`: Completes the range definition, creating a `RangeImpl` from `this.from` up to (but not including) `n`. Throws `InvalidRangeError` if `this.from` is greater than `n`.
    *   `through(n)`: Completes the range definition, creating a `RangeImpl` from `this.from` up to and including `n`. Throws `InvalidRangeError` if `this.from` is greater than `n`.

*   **`Range`**
    The primary entry point for creating ranges using a fluent API.

    **Methods:**
    *   `type func from(n)`: Starts a new range definition from `n`, returning a `RangeFrom` object.

---
# `aria.rng` Module Reference

This document provides a reference for the `aria.rng` module, which contains utilities for random number generation.

---

## Modules

### `aria.rng.mixin`

This module provides a mixin for adding common functionality to Random Number Generator (RNG) structs.

#### **Mixins**

*   **`RngRange`**
    A mixin that provides methods for generating random numbers within a specific range or selecting a random element from a list.

    **Requirements:**
    *   The struct including this mixin **must** implement an instance method `next()` that returns a random integer.

    **Methods Offered:**
    *   `in_range(low, high)`: Returns a random integer within the inclusive range `[low, high]`.
    *   `one_of(x: List)`: Returns a random element from the provided list `x`.

---

### `aria.rng.msws`

This module provides an implementation of the Middle-Square Weyl Sequence RNG.

#### **Structs**

*   **`MiddleSquareRng`**
    An object that generates pseudo-random numbers using the Middle-Square Weyl Sequence algorithm.

    **Methods:**
    *   `type func new()`: Creates a new `MiddleSquareRng` instance, seeded with the current system time.
    *   `type func new_with_params(x, s)`: Creates a new `MiddleSquareRng` instance with a specific starting value `x` and a Weyl sequence value `s`.
    *   `next()`: Returns the next pseudo-random integer in the sequence.
    *   *(Included from `RngRange`)* `in_range(low, high)`
    *   *(Included from `RngRange`)* `one_of(x: List)`

---

### `aria.rng.xorshift`

This module provides an implementation of the Xorshift RNG.

#### **Structs**

*   **`XorshiftRng`**
    An object that generates pseudo-random numbers using the Xorshift algorithm.

    **Methods:**
    *   `type func new()`: Creates a new `XorshiftRng` instance, seeded with the current system time.
    *   `type func new_with_seed(seed)`: Creates a new `XorshiftRng` instance with a specific seed value.
    *   `next()`: Returns the next pseudo-random integer in the sequence.
    *   *(Included from `RngRange`)* `in_range(low, high)`
    *   *(Included from `RngRange`)* `one_of(x: List)`

---
# `aria.string` Module Reference

This document provides a reference for the `aria.string` module, which contains utilities and extensions for working with strings.

---

## Modules

### `aria.string.classes`

This module provides methods to check if a single-character string belongs to a certain character class (e.g., digit, letter).

#### **Extensions**

*   **`extension String`**
    This extends the built-in `String` type with the following instance methods. These methods are intended to be called on strings containing only a single character.

    **Methods:**
    *   `is_digit()`: Returns `true` if the character is a numeric digit ('0'-'9'), `false` otherwise.
    *   `is_uppercase_letter()`: Returns `true` if the character is an uppercase ASCII letter ('A'-'Z'), `false` otherwise.
    *   `is_lowercase_letter()`: Returns `true` if the character is a lowercase ASCII letter ('a'-'z'), `false` otherwise.
    *   `is_letter()`: Returns `true` if the character is an uppercase or lowercase ASCII letter, `false` otherwise.
    *   `is_alphanumeric()`: Returns `true` if the character is an ASCII letter or a numeric digit, `false` otherwise.
    *   `is_whitespace()`: Returns `true` if the character is a space, newline, carriage return, or tab, `false` otherwise.

---

### `aria.string.regex`

This module provides regular expression functionality.

#### **Structs**

*   **`Regex`**
    Represents a compiled regular expression.

    **Fields:**
    *   `pattern` (String): The regular expression pattern string.

    **Methods:**
    *   `type func new(pattern: String)`: Compiles a new `Regex` from the given `pattern` string. Throws `Regex.Error` if the pattern is invalid.
    *   `any_match(text: String)`: Returns `true` if the regex matches any part of the `text`, `false` otherwise.
    *   `matches(text: String)`: Searches for every match of the regex in `text`. Returns a `List` of `Regex.Match` objects, potentially empty if no matches are found.
    *   `replace(text: String, with: String)`: Searches for every match of the regex in `text` and replaces each occurrence with `with`. Returns a `String` with all the substitutions performed.

*   **`Regex.Match`**
    Represents a single match found by a regular expression.

    **Fields:**
    *   `start` (Int): The starting byte index of the match in the input string.
    *   `len` (Int): The length of the match in bytes.
    *   `value` (String): The matched string itself.

---
# `aria.structures` Module Reference

This document provides a reference for the `aria.structures` module, which contains common data structures.

---

## Modules

### `aria.structures.map`

This module provides a hash map implementation.

#### **Structs**

*   **`Map`**
    A collection of key-value pairs, implemented as a hash map. Keys must provide a `hash()` method.

    **Methods:**
    *   `type func new()`: Creates a new, empty `Map` with a default capacity.
    *   `type func new_with_capacity(n: Int)`: Creates a new, empty `Map` with a specified initial capacity.
    *   `set(k, v)`: Sets the value `v` for the key `k`. If the key already exists, its value is overwritten.
    *   `get(k)`: Retrieves the value for key `k`. Returns `Maybe::Some(value)` if the key exists, otherwise `Maybe::None`.
    *   `remove(k)`: Removes a key and its associated value from the map.
    *   `contains(k)`: Returns `true` if the map contains the given key, `false` otherwise.
    *   `len()`: Returns the number of key-value pairs in the map.
    *   `keys()`: Returns a `List` of all keys in the map.
    *   `[](k)`: Retrieves the value for key `k`. Throws an exception if the key does not exist.
    *   `[]=(k, v)`: Sets the value `v` for the key `k`.
    *   `iterator()`: Returns an iterator that yields key-value pairs (as `Box` objects with `.key` and `.value` fields) for use in `for` loops.
    *   `prettyprint()`: Returns a string representation of the map.

---

### `aria.structures.queue`

This module provides a priority queue.

#### **Structs**

*   **`PriorityQueue`**
    A queue that orders its elements based on a priority. By default, it acts as a min-heap.

    **Methods:**
    *   `type func new()`: Creates a new, empty `PriorityQueue` that uses the default `<` operator for comparison (min-heap).
    *   `type func new_with_comparator(cmp)`: Creates a new, empty `PriorityQueue` with a custom comparator function. The function `cmp(a, b)` should return `true` if `a` has a higher priority than `b`.
    *   `push(item)`: Adds an item to the queue.
    *   `pop()`: Removes and returns the item with the highest priority from the queue. Throws an exception if the queue is empty.
    *   `peek()`: Returns the highest priority item without removing it. Returns `Maybe::Some(item)` or `Maybe::None` if the queue is empty.
    *   `len()`: Returns the number of items in the queue.

---

### `aria.structures.set`

This module provides a collection of unique items.

#### **Structs**

*   **`Set`**
    A collection that stores unique values, implemented using a `Map` internally. Items must provide a `hash()` method.

    **Methods:**
    *   `type func new()`: Creates a new, empty `Set`.
    *   `type func new_from_items(...)`: Creates a new `Set` populated with the provided arguments.
    *   `set(x)`: Adds an item to the set. If the item already exists, this has no effect.
    *   `contains(x)`: Returns `true` if the set contains the given item, `false` otherwise.
    *   `remove(x)`: Removes an item from the set.
    *   `len()`: Returns the number of items in the set.
    *   `union(other)`: Returns a new `Set` containing all items present in either this set or the `other` set.
    *   `intersection(other)`: Returns a new `Set` containing only the items present in both this set and the `other` set.
    *   `difference(other)`: Returns a new `Set` containing items present in this set but not in the `other` set.
    *   `iterator()`: Returns an iterator that yields the items in the set for use in `for` loops.

---

### `aria.structures.stack`

This module provides a last-in, first-out (LIFO) stack.

#### **Structs**

*   **`Stack`**
    A LIFO data structure.

    **Methods:**
    *   `type func new()`: Creates a new, empty `Stack`.
    *   `push(x)`: Adds an item to the top of the stack.
    *   `pop()`: Removes and returns the item from the top of the stack. Throws an exception if the stack is empty.
    *   `try_pop()`: Removes and returns the top item as `Maybe::Some(item)`, or returns `Maybe::None` if the stack is empty.
    *   `peek()`: Returns the top item without removing it, as `Maybe::Some(item)`, or `Maybe::None` if the stack is empty.
    *   `peek_at(offset)`: Returns the item at a given `offset` from the top of the stack without removing it (e.g., `peek_at(1)` looks at the second item from the top). Returns a `Maybe` value.
    *   `len()`: Returns the number of items in the stack.
    *   `is_empty()`: Returns `true` if the stack contains no items, `false` otherwise.

---
# `aria.test` Module Reference

This document provides a reference for the `aria.test` module, which contains utilities for writing and running tests.

---

## Modules

### `aria.test.test`

This module provides the core components for defining test cases and organizing them into test suites.

#### **Enums**

*   **`TestResult`**
    An enumeration representing the outcome of a test case execution.

    **Cases:**
    *   `Pass`: The test case executed successfully without any failures.
    *   `Fail(String)`: The test case failed, with the `String` payload providing a description of the failure.

#### **Mixins**

*   **`TestCase`**
    A mixin that provides common testing utilities and a `run` method for executing a test.

    **Requirements:**
    *   The struct including this mixin **must** implement an instance method `test()` which contains the actual test logic. This method is expected not to throw any errors for a passing test.
    *   Optionally, the struct may implement `setup()` and `teardown()` instance methods. If present, `setup()` is called before `test()`, and `teardown()` is called after `test()` (regardless of `test()`'s outcome).

    **Methods Offered:**
    *   `type func new()`: Creates a new instance of the test case.
    *   `run()`: Executes the test case. It calls `setup()` (if present), then `test()`, and finally `teardown()` (if present). It catches exceptions thrown by `test()` and returns a `TestResult::Fail` if the test fails or throws an exception. Returns `TestResult::Pass` on success.
    *   `assert_equal(expected, actual)`: Asserts that `expected` is equal to `actual`. Throws `ComparisonMismatch` if they are not equal.
    *   `assert_not_equal(expected, actual)`: Asserts that `expected` is not equal to `actual`. Throws `ComparisonMismatch` if they are equal.
    *   `assert_throws(f)`: Asserts that calling the function `f` throws an exception. If `f` does not throw, it throws `OperationFailure`.

#### **Structs**

*   **`TestSuite`**
    A container for organizing and running multiple `TestCase` instances.

    **Fields:**
    *   `name` (String): The name of the test suite.
    *   `tests` (List): A list of `TestCase` instances to be run.

    **Methods:**
    *   `type func new(name)`: Creates a new `TestSuite` with the given `name`.
    *   `add_test(test)`: Adds a `TestCase` instance to the suite. Returns the `TestSuite` instance for chaining.
    *   `run(silent=false)`: Executes all test cases added to the suite. Prints the result of each test and a summary of passed/failed tests. Returns the number of failed tests. If `silent` is `true`, it suppresses the test runner's output during execution.

---
# Built-in Values

This section provides a reference for the built-in values of the Aria language.

### ARIA_VERSION
A string representing the current version of the Aria language. It is usually in the format major.minor.date (e.g. 0.9.20251225 for a build of version 0.9 released on December 25, 2025). It is **not** a semantic version. It is **discouraged** to use the version number for program logic.

### `alloc(type)`
Allocates a new object of the given type with a default value.
*   **Arguments:**
    *   `type`: The type to allocate (e.g. a struct type).
*   **Returns:** A new object of the specified type.

### `arity(callable)`
Returns the arity of a callable object (function, bound function, etc.).
*   **Arguments:**
    *   `callable`: The callable object.
*   **Returns:** A struct with the following fields:
    *   `min` (Int): The minimum number of arguments.
    *   `max` (Int or `UpperBound`): The maximum number of arguments.
    *   `has_receiver` (Bool): Whether the callable has a receiver (`this`).

### `cmdline_arguments()`
Returns a list of strings representing the command-line arguments passed to the VM.
*   **Returns:** A `List` of `String`s.

### `getenv(name)`
Returns the value of an environment variable.
*   **Arguments:**
    *   `name` (String): The name of the environment variable.
*   **Returns:** `Maybe::Some(String)` if the variable is found, `Maybe::None` otherwise.

### `hasattr(object, name)`
Checks if an object has a specific attribute.
*   **Arguments:**
    *   `object`: The object to inspect.
    *   `name` (String): The name of the attribute.
*   **Returns:** `true` if the attribute exists, `false` otherwise.

### `listattrs(object)`
Returns a list of an object's attributes.
*   **Arguments:**
    *   `object`: The object to inspect.
*   **Returns:** A `List` of `String`s representing the attribute names.

### `now()`
Returns the current time as the number of milliseconds since the Unix epoch.
*   **Returns:** An `Int`.

### `prettyprint(object)`
Returns a string representation of an object.
*   **Arguments:**
    *   `object`: The object to represent.
*   **Returns:** A `String`.

### `print(object)`
Prints a string representation of an object to the console.
*   **Arguments:**
    *   `object`: The object to print.
*   **Returns:** `Unit`.

### `println(object)`
Prints a string representation of an object to the console, followed by a newline.
*   **Arguments:**
    *   `object`: The object to print.
*   **Returns:** `Unit`.

### `readattr(object, name)`
Reads the value of an attribute from an object.
*   **Arguments:**
    *   `object`: The object to read from.
    *   `name` (String): The name of the attribute.
*   **Returns:** The value of the attribute.

### `readln(prompt)`
Reads a line of input from the user after displaying a prompt.
=*   **Arguments:**
    *   `prompt` (String): The prompt to display.
*   **Returns:** A `String` containing the user's input.

### `sleep_ms(milliseconds)`
Pauses execution for a specified duration.
*   **Arguments:**
    *   `milliseconds` (Int): The number of milliseconds to sleep.
*   **Returns:** `Unit`.

### `system(command)`
Executes a shell command.
*   **Arguments:**
    *   `command` (String): The command to execute.
*   **Returns:** An `Int` object representing the exit code, with `stdout` and `stderr` attributes containing the command's output.

### `typeof(object)`
Returns the type of an object.
*   **Arguments:**
    *   `object`: The object to inspect.
*   **Returns:** A `Type` object.

### `writeattr(object, name, value)`
Writes a value to an attribute of an object.
*   **Arguments:**
    *   `object`: The object to modify.
    *   `name` (String): The name of the attribute.
    *   `value`: The value to write.
*   **Returns:** `Unit`.