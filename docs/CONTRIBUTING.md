---
---

# Contributing to Aria

First of all, thank you for considering contributing to Aria! We're excited you're here.

Whether you’re fixing bugs, writing tests, or improving documentation, every contribution counts. Your help makes Aria better for everyone!

This document provides guidelines to help you get started.

## How Can I Contribute?

There are many ways to contribute, and all of them are valuable.

*   **Reporting Bugs:** If you find a bug, please [open an issue](https://github.com/egranata/aria/issues) and provide as much detail as possible, including the version of Aria you're using (commit hash most likely), your operating system, the code that triggered the bug, and the full error message.
*   **Suggesting Enhancements:** Have an idea for a new feature or an improvement? Whether it’s a core language change, an optimization, or an update to the standard library, we’d love to hear your suggestions. Please [open an issue](https://github.com/egranata/aria/issues) to start the discussion.
*   **Improving Documentation:** Great documentation is key. If you see a typo, find something confusing, or think a section is missing, please feel free to open an issue or a pull request.
*   **Writing Code:** If you're ready to write some code, that's fantastic! Please look for an existing issue that you'd like to work on. If you're thinking of something else that needs worked on, please file a GitHub issue to coordinate your contribution and avoid duplication of effort.

Some issues are labeled [good starter bugs](https://github.com/egranata/aria/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22) or [help wanted](https://github.com/egranata/aria/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22help%20wanted%22). If you're looking to start contributing to Aria, these could be great starting places.

## Your First Code Contribution

Ready to dive in? Here’s how to set up your environment and submit your first pull request.

### 1. Fork and Clone the Repository

1.  Fork the repository on GitHub by clicking the "Fork" button.
2.  Clone your forked repository to your local machine:
    ```shell
    git clone https://github.com/YOUR_USERNAME/aria.git
    cd aria
    ```

### 2. Create a Branch

Before you do anything else, review the Getting Started section in [README.md](README.md). It should give you a compiled build of Aria and a passing test suite.

Once you know you're in a happy steady state, create a new branch for your changes. Please choose a descriptive name. If you're working on an existing issue, it's helpful to include the issue number in the branch name.

```shell
# Example for fixing issue #42
git checkout -b fix-issue-42-json-parsing-error
```

We recommend installing the [pre-commit hooks](https://pre-commit.com/#install), as they automatically run checks on your code to ensure that it adheres to the project's style and quality standards.

### 3. Write Your Code

Now you're ready to make your changes!

The Aria compiler and VM are written in Rust, as are some core parts of the Aria standard library. Most of the Aria standard library is written in Aria itself, and you should be able to make meaningful contributions to it with minimal to no Rust expertise.

*   **Coding Style:** Please try to match the existing coding style. For Rust code, we use `rustfmt` (which is run automatically by the pre-commit hooks if you install them). For Aria code, please observe the style in surrounding files.
*   **Add Tests:** If you are adding a new feature or fixing a bug, please add a corresponding test case to the `tests/` directory. This is crucial for maintaining the quality of the project.
*   **Add Documentation:** If you are adding new behavior to the language or new APIs to the standard library, update the corresponding documentation. This is crucial to allow discoverability of your feature.

### 4. Submit a Pull Request

Once your changes are ready, it's time to submit a pull request.

1.  Commit your changes with a clear and descriptive commit message.
    `git commit`

    (in your text editor of choice)

    ```
    Fix an issue with parsing nested lists in JSON

    When a nested JSON list was being parsed, the parser state was incorrectly
    set to look for a { instead of ]. Fix this by keeping track of the nesting
    structure correctly.

    Fixes #42
    ```

    There is no specific required format, as long as your commit message describes what you changed and (perhaps more importantly *why* you changed it), and adds a `Fixes` tag, if your patch is fixing a GitHub issue.

2.  Push your branch to your fork on GitHub:
    ```shell
    git push origin fix-issue-42-json-parsing-error
    ```

3.  Go to the original Aria repository on GitHub and open a pull request. If you use the github tool, `gh pr create` will also guide you through some of these steps.

4.  One of the project maintainers will review your pull request, provide feedback, and work with you to get it merged.

Thank you again for your contribution. It's an exciting time for Aria, and we're happy to have you on board!
