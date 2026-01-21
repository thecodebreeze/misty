# Contributing to Misty

Thanks for your interest in contributing to Misty! This document explains how to propose changes, report issues, and
submit pull requests in a way that’s easy to review and maintain.

Misty is an **active, under-development** project: APIs and behavior may change quickly, and we may prioritize momentum
over perfect polish. That said, we *do* care about correctness, clarity, and a clean contributor experience.

## Code of Conduct

This project has a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold it.

## Ways to contribute

You can help with:

- **Bug reports** (compiler, parser, codegen, networking runtime, CLI)
- **Feature requests** and design discussions
- **Documentation** improvements (README/wiki/docs comments)
- **Implementation work** across workspace crates:
    - `crates/misty-compiler` (CLI / compiler driver)
    - `crates/misty-parser` (parsing)
    - `crates/misty-ast` (AST types)
    - `crates/misty-codegen-rs` (Rust codegen)
    - `crates/misty-net` (networking)
    - `crates/misty-core` (shared core utilities)

## Before you open an issue

Please:

1. **Search existing issues** (open and closed) to avoid duplicates.
2. Collect a minimal repro if possible (a small IDL snippet, CLI command, logs).
3. Remove or redact any sensitive information.

### Bug reports: include

- What you expected to happen
- What actually happened
- Steps to reproduce
- Your environment:
    - OS
    - Rust version (`rustc --version`)
    - Misty version / git commit
- Relevant logs/output (trimmed)

### Feature requests: include

- The problem you’re trying to solve
- Proposed solution (even rough is fine)
- Alternatives considered
- Compatibility concerns (e.g., wire format, language targets, tooling UX)

## Security issues

If you believe you’ve found a security vulnerability, **please do not open a public issue**.

Instead, contact the maintainers privately via the contact information in the repository profile or a secure channel you
trust. If you’re unsure, open a minimal issue that asks for a security contact without including details.

## Development setup

### Prerequisites

- **Rust MSRV: 1.92.0**
    - Please keep CI and contributions compatible with Rust **1.92.0** unless the maintainers explicitly bump MSRV.
- A standard Rust toolchain with Cargo

Optional but helpful:

- `rustfmt` and `clippy` components
- A recent `git`

### Clone and build

```sh
git clone https://github.com/thecodebreeze/misty.git
cd misty
cargo build
```

### Run tests

```sh
cargo test
```

### Format and lint

```sh
cargo fmt
cargo clippy --all-targets --all-features
```

> Tip: If you’re working on a single crate, you can scope commands:
>
> ```sh
> cargo test -p misty-parser
> cargo clippy -p misty-compiler --all-targets --all-features
> ```

## Project conventions

### Keep changes focused

Smaller PRs are easier to review and merge. If you’re doing multiple unrelated changes, consider splitting them.

### Document behavior changes

If a change affects:

- the IDL syntax/semantics,
- generated APIs,
- networking behavior,
- CLI flags/output,

please include a short note in the PR description (and update docs where appropriate).

### Compatibility and versioning notes

Misty has a defined versioning policy tied to Rust versions:

- Rust **minor** version bumps are treated as **non-breaking** for Misty and result in a **minor** version bump for
  Misty crates.
- Rust **major** versions and **edition** changes are treated as **breaking** and result in a **major** version bump for
  Misty crates.

When proposing changes that may force an MSRV bump, call it out explicitly in your PR.

### Changelog updates

User-facing changes should include an update to the structured plain-text changelog in [`CHANGELOG`](CHANGELOG).

The changelog is maintained in **append-only log (AOL)** style:

- newer entries are closer to the top,
- each release is a self-contained block,
- there is no “Unreleased” section.

## AI-assisted contributions

AI-assisted contributions are welcome.

Please read [AGENTS.md](AGENTS.md) for repository-specific guidance and expectations when using AI tools (structure,
style, verification steps, etc.).

If you use AI to generate code or text, you’re still responsible for:

- correctness and safety,
- licensing/originality of what you submit,
- ensuring the change matches project conventions.

## Submitting a pull request

### 1) Create a branch

```sh
git checkout -b yourname/short-description
```

### 2) Make your changes

- Add tests when practical
- Keep formatting clean (`cargo fmt`)
- Avoid unrelated refactors in the same PR

### 3) Ensure everything passes locally

```sh
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

### 4) Open the PR

In your PR description, include:

- **What** changed
- **Why** it changed
- **How** to test
- Any follow-ups or known limitations

If your PR addresses an issue, link it (e.g., `Fixes #123`).

## Review process

Maintainers may request:

- changes to naming, structure, or API ergonomics,
- additional tests,
- documentation updates,
- splitting a PR into smaller pieces.

Please don’t take it personally—review comments are about keeping Misty consistent and maintainable.

## Getting help

If you’re not sure where to start:

- open an issue describing what you’d like to work on,
- or open a draft PR early with questions and notes.

Good “first contributions” often include:

- docs clarifications,
- improved error messages,
- small parser/codegen fixes,
- additional tests and examples.
