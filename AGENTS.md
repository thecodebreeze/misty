# AGENTS.md — AI Assistant Guidelines

This repository welcomes AI-assisted contributions. This document tells AI tools (and the humans using them) how to work
in this codebase without creating chaos.

If anything here conflicts with repository maintainers’ instructions in issues/PRs, follow the maintainers.

## Project snapshot

**Misty** is a Rust workspace implementing a modern IDL + networking framework (think Protobuf/gRPC, but built around
QUIC and WebTransport where relevant).

Workspace crates live under `crates/` (e.g. `misty-parser`, `misty-compiler`, `misty-net`, etc.).

## Rust toolchain and MSRV

- **MSRV:** Rust **1.92.0**
- Keep PRs compatible with Rust **1.92.0** unless an MSRV bump is explicitly part of the change.
- Do not introduce dependencies or language features that require a newer compiler without calling it out.

### Rust/MSRV → Misty versioning policy

When proposing changes that require a Rust toolchain bump:

- **Rust minor bump** (e.g. `1.92` → `1.93`) is **not breaking** for Misty crates → **minor** version bump in Misty
  crates.
- **Rust major bump** or **edition change** is **breaking** for Misty crates → **major** version bump in Misty crates.

(If you’re unsure whether something forces a bump, say so in the PR description.)

## Build, test, format, lint (common commands)

Run these from the repository root.

### Build

```sh
cargo build
```

### Test

```sh
cargo test
```

### Format

```sh
cargo fmt
```

### Lint (Clippy)

```sh
cargo clippy --all-targets --all-features
```

### Useful scoped commands

Work on a single crate when possible to keep feedback fast:

```sh
cargo test -p misty-parser
cargo clippy -p misty-compiler --all-targets --all-features
cargo build -p misty-net
```

### “Before opening a PR” checklist

AI-generated changes should not skip this:

```sh
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## How to make changes (AI workflow expectations)

1. **Prefer small, reviewable PRs**
    - One feature/fix per PR.
    - Avoid drive-by refactors mixed with functional changes.

2. **Preserve public API stability intentionally**
    - If you change public APIs (types, functions, CLI flags), call it out.
    - If the change impacts generated code or wire behavior, document it in the PR.

3. **Add tests when practical**
    - Especially for parser/codegen behavior. If tests aren’t possible yet, explain why and include a repro snippet.

4. **Keep docs in sync**
    - If you add a feature flag, CLI command, or behavior change: update docs as appropriate.

## Commit conventions (Semantic Commits)

Use **Conventional Commits** style messages. This helps automate changelogs and versioning.

### Format

```text
<type>(<scope>): <summary>

<body>

<footer>
```

### Common types

- `feat`: new user-facing feature
- `fix`: bug fix
- `docs`: documentation only
- `refactor`: code change that neither fixes a bug nor adds a feature
- `perf`: performance improvement
- `test`: add or update tests
- `build`: build system / tooling changes
- `ci`: CI configuration changes
- `chore`: maintenance tasks (no production code changes)

### Suggested scopes

Use the crate name or subsystem:

- `misty-parser`, `misty-compiler`, `misty-net`, `misty-ast`, `misty-core`, `codegen`, `workspace`

### Examples

```text
feat(misty-parser): support trailing commas in message fields
fix(misty-compiler): report file paths correctly on parse errors
docs(workspace): clarify MSRV and versioning policy
refactor(misty-net): simplify connection state machine
```

### Breaking changes

If a commit introduces a breaking change, use **either**:

- `!` after the type/scope, **and/or**
- `BREAKING CHANGE:` footer explaining what breaks and how to migrate.

Examples:

```text
feat(misty-net)!: change handshake API to return ConnectionId

BREAKING CHANGE: Connection::handshake now returns (ConnectionId, Session) instead of Session.
```

## SemVer guidance for Misty crates

This workspace publishes multiple crates. When changing behavior, think in **SemVer** terms:

### Patch (`x.y.Z`)

- Bug fixes
- Internal refactors
- Docs/tests (no API change)
- Performance improvements without observable behavior changes

### Minor (`x.Y.z`)

- Backwards-compatible features
- New APIs that don’t break existing code
- Backwards-compatible CLI additions

### Major (`X.y.z`)

- Any breaking API change
- Removal/rename of public items
- Behavior changes that break existing consumers
- Rust **major** version or **edition** changes (per policy)

> Note: While the project is still in `0.y.z`, treat “minor” bumps as potentially breaking in practice. Still use the
> SemVer categories above so maintainers can manage releases consistently.

### Workspace version bumps

If a change affects multiple crates, call out which crates require version bumps in the PR description. When in doubt:

- bump the crate where the public API changed,
- bump dependents only if needed (e.g., they expose the changed types in their own public API).

## Proposing version bumps (crates + releases)

AI assistants should explicitly propose SemVer bumps in the PR description whenever a change could affect consumers.

### What to include in the PR description

Add a small “Versioning” block like this:

```text
Versioning:
- misty-<crate>: patch|minor|major — <1 sentence why>
- misty-<crate>: patch|minor|major — <1 sentence why> (if multiple crates)
MSRV impact: none | requires Rust <version> (explain why)
Breaking: yes|no (if yes, include migration note)
```

### How to decide which crates to bump

- If you changed a crate’s **public API** (public structs/enums, function signatures, trait methods, feature flags that
  alter public surface): bump that crate.
- If you changed a crate’s **behavior** in a way consumers can observe (CLI output/flags, generated code output,
  protocol/wire behavior): bump that crate.
- If another crate depends on the changed crate:
    - **Only bump dependents** if they must change to compile OR if they **re-export/expose** the changed API in their
      own public API.

### Quick SemVer rules (applies per crate)

- **Patch**: bugfixes, refactors, perf, docs/tests, internal changes with no API/behavior break.
- **Minor**: new backwards-compatible features/APIs; additive CLI features.
- **Major**: breaking API changes, removals/renames, incompatible behavioral changes, protocol breaks.

> Reminder: in this repo, Rust **minor** toolchain bumps imply a **minor** bump for Misty crates; Rust **major** or *
*edition** changes imply a **major** bump.

## Maintaining the changelog (`CHANGELOG`)

This repo uses a structured **plain-text** changelog stored in `CHANGELOG` (no extension), in **append-only log (AOL)**
style:

- **Newer entries go at the top**
- Each release is a self-contained block
- No “Unreleased” section

### When to add a changelog entry

Add an entry when your PR is user-visible:

- new features (IDL/compiler/codegen/networking/CLI)
- fixes users will notice
- behavior/protocol changes
- deprecations/removals
- MSRV/toolchain policy changes

You can skip it for internal refactors, tests-only, or CI-only changes (unless it affects users).

### Format

Add a new release block at the **top** of the file:

```text
VERSION X.Y.Z (YYYY-MM-DD)
--------------------------

Added:
- ...

Changed:
- ...

Fixed:
- ...

Deprecated:
- ...

Removed:
- ...

Security:
- ...
```

Omit empty sections to keep it tidy (e.g., don’t include `Deprecated:` if there are none).

### PR expectation

In PRs that change user-facing behavior, include:

- the changelog entry in `CHANGELOG`, and
- a short “Versioning” note (which crates bump + why).

## PR template (copy/paste)

Use this structure in the PR description to keep reviews fast:

```markdown
### Summary

- <what changed>
- <what changed>

### Motivation

<why this change is needed>

### How to test

    ```textmate
    cargo fmt
    cargo clippy --all-targets --all-features
    cargo test
    ```

### Versioning

- <crate>: patch|minor|major — <why>
  MSRV impact: none | requires Rust <version> (why)
  Breaking: yes|no
  Migration notes (if breaking): <what users need to do>

### Notes for reviewers

- <key files / tricky parts>
- <follow-ups / TODOs>

```

## Safety and licensing

- Do not include secrets, tokens, private keys, or credentials (use placeholders like `<TOKEN>`).
- Do not paste code from unknown licenses. Prefer original code or properly attributed permissive sources (and confirm
  compatibility).
- Generated content should be verified by running the commands listed above.

## Where to look next

- [CONTRIBUTING.md](CONTRIBUTING.md) — full contributor guide
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — community expectations
- `crates/` — workspace crate sources
- `CHANGELOG` — release notes (plain-text, AOL style)
