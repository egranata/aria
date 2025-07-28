# The Aria Programming Language
[![Build Status](https://github.com/egranata/aria/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/egranata/aria/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Aria is a modern, dynamic scripting language. It is meant to be a "sweet spot" language, easy to pick-up and enjoyable to use.

It provides a familiar C-style syntax, with a feature set inspired by well-beloved languages such as Python and Rust. It comes with little ceremony and a focus on getting stuff done.

The standard library, while simple, has enough basic features to get you started on interesting problems.

Aria is currently only supported on Linux. Contributions for other operating systems are welcome and encouraged!

## A Taste of Aria

Aria is easy to learn. Here's a [quick example](examples/github_user.aria) that fetches data from a web API and prints the result.

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

Ready to try Aria? Want to contribute to the language? Great! For all this and more, visit [our website](https://egranata.github.io/aria/)!