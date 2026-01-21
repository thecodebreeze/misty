# Misty

Misty is a modern **IDL (Interface Definition Language)** and **networking framework** for building high-performance
distributed systems—analogous to **Protobuf/gRPC**, but designed around newer transports:

- **QUIC** for efficient, secure, low-latency communication
- **WebTransport** when targeting web clients

Misty aims to keep the *ergonomics and workflow* you’d expect from schema-driven development
(schemas + interfaces + codegen), while embracing modern networking and a cohesive toolchain.

> Status: **Active and under development.** Expect rapid iteration.

## The `mistyc` tool

Misty’s workflow centers around a single, all-in-one CLI tool: **`mistyc`**.

Planned capabilities include:

- **Code generation** from Misty IDLs (initially Rust; TypeScript planned)
- **Package management** (workspace management, registry integration)
- **Formatting** and **linting** for Misty definitions
- **Protobuf → Misty conversion** to help migrate existing projects

## Workspace crates

This repository is a Cargo workspace containing multiple crates (compiler, parser, networking, codegen, AST, etc.).  
The long-term goal is to provide a complete end-to-end experience: define interfaces once → generate types + stubs →
ship services/clients across environments.

## MSRV and versioning policy

Misty follows a clearly defined Rust/MSRV policy:

- **MSRV:** Rust **1.92.0**
- **Rust minor version bumps** (e.g. `1.92` → `1.93`) are **not considered breaking** for Misty; they result in a
  **minor version bump** for Misty crates.
- **Rust major versions** and **edition changes** are considered **breaking**; they result in a **major version bump**
  for Misty crates.

## AI-friendly development

This is an **AI-friendly** project. If you’re using AI tools to contribute, start with [AGENTS.md](AGENTS.md).

## Changelog

Release notes are tracked in [`CHANGELOG`](./CHANGELOG).

If you are using Misty as a dependency, you may also want to monitor **workspace crate versions** since the repository
contains multiple crates that may evolve at different speeds.

- **Unreleased:** changes on `main` that have not shipped in a published release yet
- **0.0.x:** early development releases; breaking changes may occur between patch versions

## Contributing

Contributions are welcome—especially around the compiler pipeline, codegen targets, networking runtime, and tooling UX.

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get involved.

## License

Licensed under **BSD-3**. See [LICENSE.md](LICENSE.md).