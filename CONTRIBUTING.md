# Contributing to Flo 🦊

First off, thank you for considering contributing to Flo! It's people like you that make open source such a great community. We welcome any and all contributions, from bug reports and documentation updates to new features.

This document provides guidelines to ensure a smooth, consistent, and collaborative development process.

## How Can I Contribute?

*   **Reporting Bugs:** If you find a bug, please open an issue and provide as much detail as possible, including your OS, the command you ran, and the full output.
*   **Suggesting Enhancements:** If you have an idea for a new feature or an improvement, please open an issue to start a discussion.
*   **Pull Requests:** We welcome pull requests! Please follow the workflow described below.

## Setting Up Your Development Environment

To get started with hacking on Flo, you'll need to set up your local environment.

### Prerequisites

Flo is a Rust project that is designed to automate Git workflows. As such, the development environment has a few key dependencies.

1.  **Rust Toolchain:** Please install the Rust toolchain by following the official instructions at [rustup.rs](https://rustup.rs). Our project uses the latest stable version of Rust.
2.  **Git:** **Git is a required dependency for running the Flo test suite.** Our integration tests create real Git repositories and execute `git` commands to ensure Flo interacts with Git correctly. Please ensure `git` is installed and available in your system's PATH.

### Initial Setup

Once the prerequisites are installed, you can set up the project:

```bash
# 1. Fork the repository on GitHub
# 2. Clone your fork to your local machine
git clone https://github.com/YOUR-USERNAME/flo.git
cd flo

# 3. Build the project to make sure everything works
cargo build --workspace

# 4. Run the test suite to confirm your setup is correct
cargo test --workspace
```

If all tests pass, your environment is ready to go!

## Development Workflow

We use the **GitFlow** methodology to develop Flo itself. This means all new development happens on branches off of `develop`.

1.  **Create an Issue:** All work should start with a GitHub Issue that describes the bug or feature.
2.  **Create a Feature Branch:** Branch off the `develop` branch.
    ```bash
    git checkout develop
    git pull origin develop
    git checkout -b feat/123-my-new-feature
    ```
3.  **Write Code & Tests:** Make your changes. Remember to add or update tests to cover your changes! Our project has two types of tests:
    * **Unit tests**: Fast, in-memory tests that have no external dependencies.
    * **Integration tests**: Slower tests that require external dependencies (like `git`) and are marked with a feature flag.
4.  **Follow Coding Standards:** To make quality checks easy and consistent, we use the `just` command runner. You can install it by following the instructions at [the `just` repository](https://github.com/casey/just).
    <br>**Before submitting a Pull Request, please run the master check command from the root of the repository:**
    ```bash
    # This single command will format, lint, and run the complete test suite.
    just check
    ```
    You can slo run individual tasks:
    * `just fmt`: Format your code.
    * `just lint`: Run the linter.
    * `just test-fast`: Run only the quick, dependency-free unit tests.
    * `just test-all`: Run the complete test suite, including integration tests.
5.  **Commit Your Changes:** We use the [Conventional Commits](https://www.conventionalcommits.org/) standard for our commit messages.
6.  **Submit a Pull Request:** Push your branch to your fork and open a pull request against the `develop` branch of the main repository. Please fill out the PR template.

## Coding Standards

*   **Formatting:** All code must be formatted with `rustfmt`.
*   **Linting:** All code must pass `cargo clippy --workspace -- -D warnings`.
*   **Error Handling:** We use `anyhow` for the main binary and `thiserror` for library crates. `unwrap()` and `expect()` are not permitted in application code.
*   **Documentation:** All public APIs in our library crates (`flo-api`, `flo-pdk`) must have comprehensive doc-tests.