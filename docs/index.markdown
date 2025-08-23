# Aria Project Documentation

Welcome to the official documentation for the Aria project.

## Table of Contents

*   [Manual](manual.md)
*   [Standard Library Reference](stdlib.md)

*   [Blog](blog/index.md)
*   [Contributing](CONTRIBUTING.md)
*   [Roadmap](ROADMAP.md)
*   [Style Guide](style_guide.md)

## What is Aria?

Welcome to Aria. Aria is a modern, dynamic scripting language. It is meant to be a "sweet spot" language, easy to pick-up and with a good balance between ease of use, safety, and flexibility. Aria's design choices help you build great programs quickly.

Aria offers:
- modern, safer error handling based on algebraic data types;
- a memory safe by default approach, leveraging the Rust ecosystem under the hood;
- a flexible programming model, with an intuitive module system, optional type checks enforced at runtime, and composition as the building block of code reuse.

Aria has a simple yet usable standard library, with date/time handling, networking, file system access, JSON support and more.

Aria is currently supported on Linux and macOS. Contributions for other operating systems are welcome and encouraged!

## A Taste of Aria

Aria is easy to learn. Here's a [quick example](https://github.com/egranata/aria/examples/github_user.aria) that fetches data from a web API and prints the result. In this example, Aria fetches user data from GitHub’s API and prints the number of public repositories for a given user. This shows how simple it is to interact with external APIs and handle dynamic data in Aria.

```aria
# github_user.aria
import Request from aria.network.request;
import JsonValue from aria.json.parser;

val whoami = "egranata";

func main() {
    val request = Request.new("https://api.github.com/users/{0}".format(whoami));
    request.headers["User-Agent"] = "AriaLang/1.0";
    val response = request.get();

    if response.status_code == 200 {
        val user_data = JsonValue.parse(response.content).flatten();
        println("User {1} has {0} public repositories.".format(user_data["public_repos"], whoami));
    } else {
        println("Failed to fetch user data. Status: {0}".format(response.status_code));
    }
}
```

Running this is as simple as:
```shell
$ aria github_user.aria
User egranata has 5 public repositories.
```

## Getting Started
Ready to try Aria? Here’s how to get up and running in just a few minutes.

### 1) Prerequisites
Install Rust and Git.

For Rust, you can use [rustup.rs](https://rustup.rs/).

For Git, you can install it via your package manager (e.g. `apt`, `dnf`) or from the [official website](https://git-scm.com/downloads).

### 2) Build From Source

1.  Clone the repository:
    ```shell
    git clone https://github.com/egranata/aria.git
    cd aria
    ```
2.  Build the project using Cargo:
    ```shell
    # This builds the compiler, VM, and all libraries in debug mode.
    cargo build --workspace
    ```
    The main `aria` executable can be run via `./aria`.

The debug mode compiles Aria with debugging information to help you identify issues during development. For faster performance, you can build it in release mode later using `cargo build --release`. If you're benchmarking Aria, we recommended using release builds.

### 3) Run the Test Suite

Before you make any changes, make sure the full test suite passes.

```shell
# This script runs all Rust unit tests and the .aria language test suite.
# It's the best way to ensure everything is working correctly.
./t
```

## How to Contribute

We welcome contributions of all kinds, from bug fixes to feature ideas to documentation improvements.

*   **Found a bug?** Please [open an issue](https://github.com/egranata/aria/issues).
*   **Want to add a feature?** Check out our [**Contribution Guide**](https://egranata.github.io/aria/CONTRIBUTING.md) for details on our development process and coding standards.

By contributing to Aria, you agree that your contributions are licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).

Not a Rust developer? Not a problem. You can still help by improving the documentation, reporting bugs, or providing feedback and suggestions!

## Documentation

The Aria language is documented at [manual.md](https://egranata.github.io/aria/manual.md) and a Standard Library reference can be found at [stdlib.md](https://egranata.github.io/aria/stdlib.md).

## License

Aria is open-source software licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt). This license allows you to freely use, modify, and distribute Aria, as long as you follow its terms.