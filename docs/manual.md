# 📘 Aria User Manual

## 👋 Welcome

This document is intended to guide you on the Aria language, from the fundamentals to advanced features. It is not meant as an introduction to computer programming.

It assumes you have Aria installed, and know how to write code in a text editor and run the Aria interpreter on your system. It does not assume that you know Rust or are interested in learning it.

## 📌 Fundamentals

Aria programs contain a `main` function, defined like

```
func main() {
    # code goes here
}
```

A program can contain multiple functions, but execution starts from `main`. A very simple program is the canonical Hello World

```
func main() {
    println("Hello World");
}
```

Save it to `hello.aria` and run it as `aria hello.aria`. It should print `Hello World` and return. There is no need to provide a return value from `main`.

To declare a variable, use `val`, as in

```
val x = 1;
val y = "Hello World";
val z = 3.14f;
```

Variables are mutable. Aria does not provide an immutable value construct.

Basic arithmetic works as one would expect

```
println(x + 1); # prints 2
println(3 + 4 * 5 - 1); # prints 22
```

Integers are signed 64-bit values and they wrap around, e.g.

```
func main() {
    val x = 0x7FFFFFFFFFFFFFFF;
    println(x); # prints 9223372036854775807
    println(x+1); # prints -9223372036854775808
}
```

## 📋 Lists

Lists are a builtin data type in Aria. They are defined by a list literal, e.g.

```
val l = [1, "hello", 3.14f, false];
```

A list can contain heterogeneous elements of any Aria type. It is dynamically resizable, by appending new elements:

```
l.append(5); # l is now [1, "hello", 3.14f, false, 5];
```

The size of a list can be obtained with `l.len()` and individual elements can be indexed directly from 0 to `len()-1`,

```
println(l[0]); # prints 1
l[1] = "hi there";
println(l[1]); # prints hi there
```

Lists can contain other lists

```
val l = [1,2,3];
val ll = [l,4];
println(ll[0][0]); # prints 1
```

Multidimensional arrays are not provided.

List concatenation uses the `+` operator, so

```
println([1,2]+[3,4]); # prints [1,2,3,4]
```

## 🧵 Strings

Strings can be concatenated with the `+` operator

```
println("Hello " + 'World'); # prints Hello World
```

and string repetition uses the `*` operator

```
println("Chugga " * 2 + "Choo " * 2); # prints Chugga Chugga Choo Choo
```

Strings literals can use either `"` or `'` quotes, which makes some constructs easier to write

```
val quote = 'He said "Tu quoque, fili mi?"';
```

Strings can also be indexed with square brackets, but only for reading

```
val x = "hello";
println(x[0]); # prints h
```

String offer a formatting function, that allows interploating the values of variables or expression inside text. For example

```
val name = "Eric";
val year = 2025;
println("My name is {0} and the year is {1}".format(name, year)); # My name is Eric and the year is 2025
```

## 🔁 Control Flow

Common control flow statements and operators are available in Aria.

```
val x = 3;
if x == 3 {
    println("yes!");
} else {
    println("no!");
}
```

Alternative branches of an `if` statement are labeled `elsif`, as in

```
val x = 3;
if x == 1 {
    println("one");
} elsif x == 2 {
    println("two");
} elsif x == 3 {
    println("three"); # prints three
} else {
    println("I don't know");
}
```

There is no need to parenthesize the condition of an `if` clause. The `else` clause is optional.

`while` loops also work as expected.

```
val x = 1;
while x < 10 {
    println(x); # will print 1,2,3...
    x += 1;
}
```

Only boolean values and expressions, can be used in control flow, for example the following is illegal:

```
val x = 3;
while x {
    x -= 1;
    println(x);
}
```

`for` loops are used to iterate all over a container, for example a list.

```
val l = [2,4,6,8];
for x in l {
    println(x); # prints 2, 4, 6, 8
}
```

Loops can be exited with `break`, or jump to the next iteration with `continue`, much like in other languages in the C family.

Both `for` and `while` loops can optionally accept an `else` clause, which is executed if the body of the loop is never taken.

```
for x in [] {
    println(x);
} else {
    println("This list is empty!"); # prints This list is empty
}
```

```
val x = 0;
while x > 0 {
    println("x is positive");
    x -= 1;
} else {
    println("x is <= 0"); # prints x is <= 0
}
```

A ternary operator is provided with very similar behavior to its C counterpart:

```
val num = 3;
val str = num == 1 ? "one" : num == 2 ? "two" : "three";
println(str); # prints three
```

## 🧑‍💻 Functions

Functions are defined with the `func` keyword, and can take zero or more arguments. They are invoked in the usual manner, with their name followed by parentheses.

```
func return_answer() {
    return 42;
}

func add_numbers(x,y) {
    return x + y;
}

func main() {
    println(return_answer()); # prints 42
    println(add_numbers(3,4)); # prints 7
}
```

Functions may not return a value if they are not used as expressions, such as

```
func print_answer() {
    println(42);
}

func main() {
    print_answer(); # prints 42
}
```

Functions can accept 0 or more required arguments, 0 or more optional arguments and possibly a variable number of arguments at the end. Variable arguments are stored in a list provided to the function named `varargs`

```
func add_all_values(x, ...) {
    for arg in varargs {
        x += arg;
    }
    return x;
}

func main() {
    println(add_all_values(1,2,4,6,8)); # prints 21
    println(add_all_values(5)); # prints 5
}
```

Optional arguments are given by name and a default value

```
func add(x,y=1) {
    return x + y;
}

func main() {
    println(add(3)); # prints 4
    println(add(2,3)); # prints 5
}
```

Closures are defined with a `|args| => { body }`. Captures (if any) are implicitly handled by the Aria VM

```
func double(x) {
    return x + x;
}

func call_f(f, x) {
    return f(x);
}

func main() {
    val answer = 42;

    println(call_f(double, 12)); # prints 24
    println(call_f(|x| => { return x + 1; }, 3)); # prints 4
    println(call_f(|x| => {return x + answer; }, 2)); # prints 44
}
```

One-line functions are a shorthand form for functions that consist of a single return expression. Instead of writing a full block, you can use = after the declaration:
```
func sum(x, y) = x + y;
```
This is equivalent to writing a normal function that returns `x+y`.

## 🧱 Structs

Structs are defined as a set of operations, not data. For example

```
struct Foo {
    func blah() {
        println("I am a Foo");
    }
}

func main() {
    val f = alloc(Foo);
    f.blah();
}
```

Instances of structs are created with `alloc`, which returns a new object of the struct type.

For builtin types, `alloc` returns a default value (for example, 0 for integers and "" for strings). Some types (e.g. functions, enums, ...) cannot be instantiated with `alloc` because there is no obvious default for such values.

Instance functions are defined with the same syntax as free functions.  It is not possible to overload methods.

To access fields (data) on a struct instance, one directly reads or writes the field.

```
struct Foo {
    func blah() {
        println("I am a Foo - my value is {0}".format(this.x));
    }
}

func main() {
    val f = alloc(Foo);
    f.x = 5;
    f.blah(); # prints I am a Foo - my value is 5
}
```

It is possible to define methods that interact with the struct type instead of any given instance. For example

```
struct Foo {
    type func blahblah() {
        println("I am the Foo struct");
    }
}

func main() {
    Foo.blahblah(); # prints I am the Foo struct
}
```

The common use of `type` methods is to write constructors for structs

```
struct Foo {
    type func new(x) {
        return alloc(This) {
            .x = x
        };
    }

    func blah() {
        println("I am a Foo - my value is {0}".format(this.x));
    }
}

func main() {
    val f = Foo.new(5);
    f.blah(); # prints I am a Foo - my value is 5
}
```

By convention, construtors are named `new`, or `new_with_X` if there are multiple possible constructors supplying different arguments.
This pattern allows writing fields into a struct before the caller can access its operation, so objects are well-formed by definition.

Structs can contain each other, but not inherit each other.

When printing objects of struct type, if a `prettyprint` method is defined, `format` and `println` call it and expect it to return a string that represents the object.

```
struct Foo {
    type func new(x) {
        return alloc(This) {
            .x = x
        };
    }

    func prettyprint() {
        return "Foo({0})".format(this.x);
    }
}

func main() {
    val f = Foo.new(5);
    println(f); # prints Foo(5)
}
```

## 📑 Enumerations

Enumerations allow describing a closed set of possible values

```
enum TaskStatus {
    case NotStarted,
    case InProgress,
    case Blocked,
    case Completed
}

func main() {
    val ts1 = TaskStatus::Completed;
    val ts2 = TaskStatus::InProgress;
}
```

Enumeration cases can also contain a payload. Each case can contain at most one value, which can be of any type, including a struct. It is possible to define structs inside enums, e.g.

```
enum TaskStatus {
    struct BlockedReason {
        type func new(reason: String) {
            return alloc(This) {
                .reason = reason,
            }
        };
    }

    case NotStarted,
    case InProgress(Int),
    case Blocked(TaskStatus.BlockedReason),
    case Completed
}

func main() {
    val ts1 = TaskStatus::Completed;
    val ts2 = TaskStatus::InProgress(45);
}
```

To check if an enumeration value is a specific case, helper `is_X` methods are provided for each case. For cases with payload, `unwrap_X` methods provide the payload, if the value is of that case.

```
enum TaskStatus {
    case NotStarted,
    case InProgress(Int),
    case Blocked,
    case Completed
}

func main() {
    val ts1 = TaskStatus::Completed;
    val ts2 = TaskStatus::InProgress(45);

    println(ts1.is_Blocked()); # prints false
    println(ts2.is_InProgress()); # prints true
    println(ts2.unwrap_InProgress()); # prints 45
}
```

It is a runtime error to call `unwrap_X` on an enumeration value not representing that case.

One can also use the `match` statement to extract case and value from an enum, for example

```
enum TaskStatus {
    case NotStarted,
    case InProgress(Int),
    case Blocked,
    case Completed
}

func main() {
    val ts1 = TaskStatus::Completed;
    val ts2 = TaskStatus::InProgress(45);

    match ts2 {
        isa TaskStatus and case InProgress(x) => {
            println("In Progress, {0}% completed".format(x));
        },
        isa TaskStatus and case Blocked => {
            println("Blocked");
        },
        # ...
    } # prints In Progress, 45% completed
}
```

The two clauses in `match` above check that ts2 is of type `TaskStatus` and the case that it covers. A `case` check would only match the name of the case, but not the type (multiple enums could have the same case name). For enums with payload, the payload can also be extracted in the `case` clause by providing an identifier.

More broadly, a match statement can also be used to check some simple comparisons, for example

```
func main() {
    val x = 3;

    match x {
        >= 5 => { println("A very large number"); },
        > 3 => { println("A good number"); },
        == 3 => { println("It's three!"); }, # prints It's three!
        > 0 => { println("A small positive number"); }
    } else {
        println("not sure!");
    }
}
```

Valid operators are ==, !=, isa, >, >=, <, >= and they can be combined with `and` clauses:

```
func main() {
    val x = 3;

    match x {
        isa String and == "hello world" => { println("hi there!"); },
        isa Int and >= 4 => { println("Four or more"); },
        isa Int and > 0 and < 5 => { println("It might be three?"); }, # prints It might be three?
    }
}
```

## ⁉️ Maybe

`Maybe` is an enum that represents a potentially missing value. It is defined as

```
enum Maybe {
    case Some(Any),
    case None,
}
```

As an enum, it can be used in `match` or checked with `is_X` helpers. APIs where a value may be returned or not, and neither condition is an error, use `Maybe` to represent that fact.

## 🗺️ Maps

Maps are provided by the Aria standard library. To import the Map data type, use `import Map from aria.structures.map;`. This gives access to the `Map` data type.

Values can be inserted into a map by key

```
import Map from aria.structures.map;

func main() {
    val m = Map.new();
    m[1] = "one";
    m[2] = "two";
}
```

and retrieved by key

```
import Map from aria.structures.map;

func main() {
    val m = Map.new();
    m[1] = "one";
    m[2] = "two";

    println(m[1]); # prints one
}
```

It is a runtime error to try to retrieve a missing key with the square brackets operator. In that case, use `get`, which returns `Maybe`.

```
import Map from aria.structures.map;

func main() {
    val m = Map.new();
    m[1] = "one";
    m[2] = "two";

    println(m.get(3).is_None()); # prints true
}
```

Maps can be iterated with `for` loops, and they return pairs of key and value

```
import Map from aria.structures.map;

func main() {
    val m = Map.new();
    m[1] = "one";
    m[2] = "two";

    for kvp in m {
        println("key = {0} value = {1}".format(kvp.key, kvp.value)); # prints key = 1 value = one key = 2 value = two
    }
}
```

Custom types can be used as keys in maps, as long as they define a `func hash()`.

## 🏷️ Object Initialization

Aria offers a quick syntax to write multiple values into the same object. Any expression can be followed by a write-list, e.g.

```
val x = [] {
    [0] = 1,
    [1] = 2,
    [2] = 3
};
```

will initialize a list with [1, 2, 3].

It can also be done for structs, for example

```
val x = Box() {
    .a = 1,
    .b = 2,
    .c = 3,
};
```

which will create an object with fields `a`, `b`, and `c`.

Indices and fields can be freely mixed:

```
val m = Map.new() {
    ["hello"] = "world",
    .something = "else",
    ["foo"] = "bar",
};

println(m); # prints Map([foo]->bar, [hello]->world)
println(m.something); # prints else
```

As a shortcut, a field can be initialized by a local variable of the same name without duplicating the name, as in

```
struct StringWrapper {
    type func new(msg) = alloc(This) { .msg };

    func prettyprint() { return this.msg; }
}

println(StringWrapper.new("hello world")); # prints hello world
```

This syntax is most often used for initializing objects and containers, but it is generally available:

```
println("x"{.hello = "world"}.hello); # prints world
```

Writes are performed in the order they are provided, and duplicated writes to the same index or name are not discarded. In general, the user should expect a "last write wins" behavior.

## 🚛 Extensions

Extensions allow to add new functions to already defined types. They are introduced by the `extension` keyword, and they otherwise look the same as a definition of a type

```
struct Counter {
    type func new() {
        return alloc(This) {
            .x = 0,
        };
    }
}

extension Counter {
    func add(x) {
        this.x += x;
        return this.x;
    }
}

extension Counter {
    func increment() {
        return this.add(1);
    }
}

func main() {
    val c = Counter.new();
    c.add(2);
    println(c.increment()); # prints 3
}
```

`add` and `increment` operate as if they were defined within the body of `Counter`'s initial definition.

Enums can also be extended the same way

```
enum Temperature {
    case Celsius(Float),
    case Fahrenheit(Float)
}

extension Temperature {
    func to_f() {
        match this {
            case Fahrenheit => { return this; },
            case Celsius(c) => {
                return Temperature::Fahrenheit(c*1.8f+32);
            }
        }
    }

    func prettyprint() {
        match this {
            case Fahrenheit(f) => {
                return "{0} F".format(f);
            }, case Celsius(c) => {
                return "{0} C".format(c);
            }
        }
    }
}

func main() {
    val tmp = Temperature::Celsius(32.0f);
    val tmp_f = tmp.to_f();
    println(tmp_f); # prints 89.6 F
}
```

## 🏈 Exceptions

Exceptions can be used to generate out-of-band control flow. Any Aria object can be thrown and caught. Exceptions cause unwinding of the stack until a frame catches, or there are no more frames left, at which point it is handled by the VM itself.

```
func main() {
    try {
        println(9 / 3); # prints 3
        println(2 /0);
    } catch e {
        println(e); # prints division by zero
    }
}
```

`catch` cannot discriminate on the type of the exception or its parameters. If a `catch` block is unable to resolve an exception, it can throw it again.

The error handling philosophy of Aria is generally inspired by Rust and Midori:
- Some scenarios are expected and anticipated (e.g. getting an element out of a Map, but there is nothing with that key); for cases like these return Maybe::None or a similar placeholder "missing" value;
- Some scenarios are erroneous, but can be recovered from (e.g. file not found, network connection failed); in these cases throw an exception and handle it somewhere else;
- Some errors are beyond recovering (e.g. the VM expected two operands but only one is present on the stack); in these cases the VM itself will throw a fatal error, or you can assert in your code. `assert` fails by throwing a non-recoverable VM error.

Aria itself defines a set of common exceptions in the `RuntimeError` enum:

- DivisionByZero: see example above;
- EnumWithoutPayload: you attempted to extract payload from an enum value that has none;
- IndexOutOfBounds: attempting to access an element beyond the end of a container;
- MismatchedArgumentCount: called a function with fewer/more arguments than it needed;
- NoSuchCase: attempting to access an enum's case that does not exist;
- NoSuchIdentifier: attempting to read/write a value that does not exist;
- OperationFailed: some task could not be completed for reasons outside of Aria's control (e.g. running an external program);
- UnexpectedType: an operation required a value of one type, but a value of a different one was provided

These values can be reused by user code, or new exception types can be created. The usual pattern for an exception is to create an ad-hoc struct

```
struct MyException {
    type func new(msg) {
        return alloc(This) {
            .msg = msg,
        };
    }

    func prettyprint() {
        return "{0}".format(this.msg);
    }
}
```

Possibly, the exception itself can craft a message based on arguments provided to it, if that makes sense in the specific case. An enumeration can be used to discriminate if an error can occur for different causes, and it's generally a better pattern than using an integer error code.

If an exception is not handled, the VM itself dumps the exception information and a stack trace. For example:

```
func do_division(x,y) {
    return x / y;
}

func complex_math(x,y) {
    val a = x + y;
    val b = x - y;
    val c = x * y;
    val d = do_division(x,y);

    return a + b + c - d;
}

func main() {
    println(complex_math(7,0));
}
```

will fail with an exception and print out

```
Error: division by zero
    ╭─[/tmp/program.aria:2:12]
    │
  2 │     return x / y;
    │            ──┬──
    │              ╰──── here
    │
  9 │     val d = do_division(x,y);
    │                        ──┬──
    │                          ╰──── here
    │
 15 │     println(complex_math(7,0));
    │                         ──┬──
    │                           ╰──── here
────╯
```

## 🎮 Operators

Structs and enums can overload a subset of operators to provide custom behavior. The resolution rules are similar to those of Python, without the support for inheritance.

The operators that can be overloaded are as follows:

| Operator Symbol | Name |
| ------------- | ------------- |
| `+` | `add` |
| `-` | `sub` |
| `*` | `mul` |
| `/` | `div` |
| `%` | `rem` |
| `<<` | `lshift` |
| `>>` | `rshift` |
| `==` | `equals` |
| `<` | `lt` |
| `>` | `gt` |
| `<=` | `lteq` |
| `>=` | `gteq` |
| `&` | `bwand` |
| `\|` | `bwor` |
| `^` | `xor` |
| `u-` | `neg` |
| `()` | `call` |
| `[]` | `read_index` |
| `[]=` | `write_index` |

Operators are overloaded by an `operator` declaration:

```
struct Integer {
    type func new(n) {
        alloc(This){
            .n = n,
        };
    }

    operator %(rhs) {
        if rhs isa Integer {
            Integer.new(this.n % rhs.n);
        } elsif rhs isa Int {
            Integer.new(this.n % rhs);
        } else {
            throw alloc(Unimplemented);
        }
    }

    func prettyprint() {
        return "{0}".format(this.n);
    }
}

func main() {
    val x = Integer.new(26);
    println(x % 4); # prints 2
}
```

If an operator does not support a combination of operands, it can throw `Unimplemented`. For a binary operator, if the type of the second operand is different, Aria will attempt to invoke the reverse operator. For operators other than equality, the syntax to define a reverse operator is `reverse operator`. For equality, one simply defines `operator ==` on the other type.

Square bracket access is defined by means of `operator [](index)` and `operator []=(index, value)`. The value of the index does not have to be an integer (e.g. `Map`), however only one index can be supported in this version of Aria.

Defining `operator ()` allows objects to be called as if they are functions, e.g.

```
struct CallMe {
    type func new() { return alloc(This); }
    operator ()(x,y,z) {
        println("You called me? x={0} y={1} z={2}".format(x,y,z));
        return x + y + z;
    }
}

func main() {
    val c = CallMe.new();
    println(c(1,2,3)); # prints You called me? x=1 y=2 z=3 followed by 6
}
```

Operator definitions generate an implementing function named `_op_impl_<name>`. While this is technically a part of the contract between the compiler and the VM, it is documented here because it can be useful to refer to the function underlying the operator in some cases, e.g. to implement a commutative reverse operator

```
struct Foo {
    operator + (rhs) {
        return 42;
    }

    reverse operator + (lhs) {
        return this._op_impl_add(lhs);
    }
}

func main() {
    println(12 + alloc(Foo)); # prints 42
}
```

To help with defining a coherent set of comparison operators, the standard library provides a `TotalOrdering` mixin at `aria.ordering.compare`.

## 👮 Guards

A guard statement is used to create a scoped block that can deallocate some managed resource on exit.

```
struct LoggedTask {
    type func new(op) {
        println("Starting {0}...".format(op));
        return alloc(This) {
            .op = op,
        };
    }

    func guard_exit() {
        println("{0} completed".format(this.op));
    }
}

func main() {
    guard op1 = LoggedTask.new("first task") { # prints Starting first task...
        # do the thing
    } # prints first task completed

    guard op2 = LoggedTask.new("second task") { # prints Starting second task...
        # do the thing
    } # prints second task completed
}
```

In the general case, objects are deallocated transparently by the Aria VM, and there is no way to control the time and flow of the deallocation.

If an object needs to execute some custom cleanup, a `guard` block is the right way to assign additional behavior independent of the general VM deallocation. Note that an object can live outside of its guard block, and it's up to the object to respond safely to that.

## 🥗 Mixins

Mixins can be used to insert new behavior into existing types. Aria does not provide inheritance, but some cases can instead be expressed via a `mixin`.

```
mixin Double {
    func double(x) {
        return 2 * x;
    }
}

struct Foo {
    include Double

    type func new() {
        return alloc(This);
    }
}

func main() {
    val f = Foo.new();
    println(f.double(5)); # prints 10
}
```

`Foo` includes all the member functions of mixin `Double`. Functions in a mixin can use other functions of their type, or the mixin, and can refer to `this`. At runtime, mixin functions see the type of the object they are included in, not the type of the mixin.

```
mixin Double {
    func double() {
        return 2 * this.x;
    }
}

struct Foo {
    include Double

    type func new(x) {
        return alloc(This) {
            .x = x,
        };
    }
}

func main() {
    val f = Foo.new(5);
    println(f.double()); # prints 10
}
```

```
mixin Double {
    func double() {
        return 2 * this.x;
    }
}

struct Foo {
    include Double

    type func new(x) {
        return alloc(This) {
            .x = x,
        };
    }
}

func main() {
    val f = Foo.new(5);
    println(f isa Foo); # prints true
    #println(f isa Double); # runtime error, the mixin is not a type
}
```

Mixins may have requirements of their types, for example `Double` expects to be included in types that have a `.x` member value of a type that can be multiplied by 2. There is no way to encode those requirements, usually they are expressed as comments in the mixin itself.

A mixin can be included in multiple types, and a type can include multiple mixins. Mixins can be included in the type definition or an extension of it.

## 💌 Sigils

Sigils are user-defined postfix operators that allow for more expressive and readable code. They provide a way to create custom unary operators that operate on the value immediately to their left.

A sigil is written as an at followed by a sequence of letters: `@letters`. When used in an expression, the sigil appears after the value it operates on.
```
value_sigil
```
This is equivalent to calling a function named `sigil` with `value` as the argument.
Before using a sigil, you must register using the `register_sigil` built-in function, which takes two arguments:
1. The sigil name (as a string)
2. The function to call when the sigil is used
```
register_sigil("sigil_name", function_to_call);
```

Complete example:
```
struct Temperature {
    func from_fahrenheit(f: Float) -> Temperature { /* ... */ }
    func from_celsius(c: Float) -> Temperature { /* ... */ }
    func to_fahrenheit() -> Float { /* ... */ }
    func to_celsius() -> Float { /* ... */ }
}

let temp1 = Temperature.from_fahrenheit(77.0);  // 77°F
let temp2 = Temperature.from_celsius(25.0);     // 25°C

let fahrenheit_value = temp2@to_f;    // Convert 25°C to Fahrenheit
let celsius_value = temp1@to_c;       // Convert 77°F to Celsius
```

## 🎡 Iterators

`for` loops work by leveraging iterators. An iterator is an object that has a `next` method with no arguments. This method is expected to return an object with a specific layout:

- if the iteration is complete, a field named `done` with value `true`;
- otherwise, a field named `done` with value `false`, and a field named `value`, whose value is the next element in the iteration

This mechanism allows for finite or infinite sequences, for values to be pre-computed or dynamically generated.

The `in` expression of a for loop is assumed to be a container of some kind, so the `iterator` method is called on it, and its return is used as the actual iterator.

```
struct SampleIterator {
    type func new() {
        return alloc(This) {
            .num = 0,
        };
    }

    instance func iterator() {
        return this;
    }

    instance func next() {
        if this.num == 5 {
            return Box() {.done = true};
        } else {
            this.num += 1;
            return Box{.done = false, .value = this.num};
        }
    }
}

func main() {
    for n in SampleIterator.new() {
        println(n); # prints 1,2,3,4,5
    }
}
```

Iterators for common types (e.g. Map, List) include the `Iterable` mixin (defined at `aria.iterator.mixin`). This mixin allows using common functional operators on an iterator (`where` (aka `filter`), `map` and `reduce`). Iterators can get pre-written implementations of these behaviors with the `Iterator` mixin from the same module. For example

```
import Iterator from aria.iterator.mixin;
import Iterable from aria.iterator.mixin;

struct SampleIterable {
    type func new() {
        return alloc(This);
    }

    instance func iterator() {
        return SampleIterator.new();
    }

    include Iterable
}

struct SampleIterator {
    type func new() {
        return alloc(This) {
            .num = 0,
        };
    }

    instance func next() {
        if this.num == 5 {
            return Box() {.done = true};
        } else {
            this.num += 1;
            return Box{.done = false, .value = this.num};
        }
    }

    include Iterator
}

func main() {
    for n in SampleIterable.new().where(|x| => x > 2).map(|x| => x + 1) {
        println(n); # prints 4,5,6
    }
}
```

This is a slightly more complex but realistic example where the iterable and the iterator are different objects. Using the `Iterable` mixin, it is possible to compose operations, so one can `where`, then `map`, then `map`, ... and so on freely.

## 📦 Modules

Any Aria file can be a module.

Modules are imported with the `import` statement, which follows a dotted structure, e.g. `import aria.rng.xorshift;`. `aria` is the name of the Aria standard library module, which contains submodules defined via filesystem paths. Documentation for the Aria library is contained in [stdlib.md](stdlib.md).

The following algorithm is used to resolve where to look for imports:
- if `ARIA_LIB_DIR` is defined, it is a list of paths separated by the platform separator; the system looks in each path in the order provided until the module is found;
- if there is a `lib/aria` directory next to the running binary, then the system looks in `lib/` for modules;
- if there is a `lib/aria` directory in the parent directory of the running binary, then the system looks in `lib/` for modules.

If running on Linux, four additional paths are searched:
- `/usr/local/aria<version>/lib` which is used if it contains an `aria` subdirectory;
- `/usr/local/aria/lib` which is used if it contains an `aria` subdirectory;
- `/usr/lib/aria<version>` which is used if it contains an `aria` subdirectory;
- `/usr/lib/aria` which is used if it contains an `aria` subdirectory.

If running on macOS, four additional paths are searched:
- `/opt/homebrew/opt/aria<version>/lib` which is used if it contains an `aria` subdirectory;
- `/opt/homebrew/opt/aria/lib` which is used if it contains an `aria` subdirectory;
- `/usr/local/opt/aria<version>/lib` which is used if it contains an `aria` subdirectory;
- `/usr/local/opt/aria/lib` which is used if it contains an `aria` subdirectory.

If `ARIA_LIB_DIR_EXTRA` is defined, it is a list of paths separated by the platform separator; each existing directory in that list is added to the end of the search path.

If no valid path exists, import modules cannot be located. Running Aria without its standard library is not a supported configuration.

As an example, `aria.rng.xorshift` is defined in `lib/aria/rng/xorshift.aria`. Directories may contain other directories, or files, or a combination thereof. A directory is scanned by virtue of its name being used, and does not need contain any marker files to be recognized as a module. Directories may be arbitrarily nested.

A module can be imported more than once, but only the first import will load the module, others will act as a no-op. An import of a module brings that module into the visible set of symbols, and names inside the module can be referenced via a fully-dotted path. For example

```
import aria.rng.xorshift;

func main() {
    val rng = aria.rng.xorshift.XorshiftRng.new();

    println(rng.next()); # will print a random value
}
```

You can structure your own projects the same way. Consider this layout:

```
my_project/
├── main.aria
└── my_lib/
    └── utils.aria
```

To use a function from `utils.aria` inside `main.aria`, you would set `ARIA_LIB_DIR` to include `my_project` and then write `import my_lib.utils;`.

To avoid dotted notation, one can import symbols directly from a module, e.g.

```
import XorshiftRng from aria.rng.xorshift;

func main() {
    val rng = XorshiftRng.new();

    println(rng.next()); # will print a random value
}
```

will work exactly the same. It is substantially equivalent to

```
import aria.rng.xorshift;

val XorshiftRng = aria.rng.xorshift.XorshiftRng;

func main() {
    val rng = XorshiftRng.new();

    println(rng.next()); # will print a random value
}
```

i.e. the entire module is imported, and one specific symbol is lifted from it.

It is possible to import multiple names from a module at once, or in separate statements.

Extensions in a module cannot be imported by name, and are always brought in when the module is imported.
