# Flo 🦊
**Your friendly, interactive guide through the streams of GitFlow.**

Flo is a modern, cross-platform command-line tool designed to bring speed, safety, and a bit of delight to your GitFlow workflows. It's more than just a script-runner; Flo is a visual assistant that takes over your terminal to provide a clear, step-by-step overview of every operation, turning complex branching logic into a calm, guided experience.
<br>
<!-- TODO: Add a high-quality animated GIF of Flo in action once available -->
<!-- ![Flo in Action](https://path/to/your/demo.gif) -->

## The Flo Philosophy
GitFlow is a powerful branching model, but it can be verbose and error-prone. Flo is built on a few core principles to solve this:
* ✨ Visual & Interactive: Instead of a black box of scripts, Flo provides a full-screen Terminal UI (TUI) that shows you exactly what's happening, step-by-step.
* 🧠 Intelligent & Proactive: Flo assists you. It suggests the next version number, helps you onboard new projects, and provides clear guidance when things go wrong, like a merge conflict.
* 🛡️ Safe & Recoverable: With features like pre-command hooks for running tests and crash recovery, Flo helps protect your repository from common mistakes.
* 🔌 Extensible for Your World: A language-agnostic plugin system for version bumping means Flo can adapt to your project's tooling, whether it's Maven, NPM, UV, or something else.

## Current Status: In Early Development 🚧
Flo is currently in the very early stages of development. The vision is set, and the foundation is being built.
* Phase 1: The Core Logic: Building the non-interactive CLI "skeleton".
* Phase 2: The Visual Experience: Implementing the core TUI.
* Phase 3: Configuration & Plugins: Making Flo extensible.
* Phase 4: Robustness & Power: Adding safety features like crash recovery.
* Phase 5: Documentation & Release: Preparing for the first official launch.

You can follow our progress by checking the Issues and Project Board.

## Features (The Vision)
Here is a look at what Flo is being built to do:
* Full GitFlow Automation: init, feature, release, and hotfix workflows.
* Cross-Platform Binary: A single, easy-to-install executable for Windows, macOS, and Linux.
* Interactive TUI: A docker-compose-like experience for every command.
* Pluggable Versioning: An extensible system for bumping versions in files like pom.xml, package.json, and pyproject.toml.
* Interactive Onboarding: A smart init command that detects your project type and helps you generate a configuration file.
* Guided Conflict Resolution: A paused state that helps you safely navigate and resolve merge conflicts.
* Pre/Post Hooks: Run your own scripts (like tests or linters) before or after operations.
* Crash Recovery: Resume a failed multi-step operation with a --continue flag.

## Installation
**Note**: Flo is not yet available for installation. This section will be updated once the first release is published.

## Contributing
This is an open-source project, and suggestions are appreciated. 
Currently, this is a personal project, but we will update this section when we are ready to invite contributors.
More information for when you want to start contributing can be found in [CONTRIBUTING.md](CONTRIBUTING.md)

## License
This project is licensed under the terms of the [MIT License](LICENSE).
