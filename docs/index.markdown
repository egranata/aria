# Aria Project Documentation

Welcome to the official documentation for the Aria project.

## Table of Contents

*   [Manual](manual.md)
*   [Standard Library](stdlib.md)
*   [Roadmap](ROADMAP.md)
*   [Contributing](CONTRIBUTING.md)

## What is Aria?

Aria is a modern, dynamic scripting language. It is meant to be a "sweet spot" language, easy to pick-up and enjoyable to use.

It provides a familiar C-style syntax, with a feature set inspired by well-beloved languages such as Python and Rust. It comes with little ceremony and a focus on getting stuff done.

The standard library, while simple, has enough basic features to get you started on interesting problems.

Aria is currently only supported on Linux. Contributions for other operating systems are welcome and encouraged!

## A Taste of Aria

Aria is easy to learn. Here's a [quick example](https://github.com/egranata/aria/examples/github_user.aria) that fetches data from a web API and prints the result.

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

**1. Prerequisites**

Install Rust and Git. For Rust, you may consider [rustup.rs](https://rustup.rs/), and Git can be generally downloaded via your distribution or from the [official website](https://git-scm.com/downloads).

**2. Build From Source**

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

**3. Run the Test Suite**

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

## Documentation

The Aria language is documented at [manual.md](https://egranata.github.io/aria/manual.md) and a Standard Library reference can be found at [stdlib.md](https://egranata.github.io/aria/stdlib.md).


## License

Aria is open-source software licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).
