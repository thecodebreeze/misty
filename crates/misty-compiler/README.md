# Misty Compiler

Misty is an IDL and network framework for building high-performance distributed systems. This crate contains the
compiler for Misty IDLs.

    This crate is under heavy active development.
    Use at your own risk.

The compiler works by generating code for any supported language based on your Misty workspace. For now the only
language supported is Rust, but there are plans to add TypeScript support soon thanks to
[oxc](https://crates.io/crates/oxc).

Below is a simplified list of features that are implemented and planned for implementation:

- [ ] Generate Rust code from Misty IDLs
- [ ] Versioning support
- [ ] Partial compilation support
- [ ] Generate TypeScript code targeting the server-side applications
- [ ] Generate TypeScript code targeting the client-side applications
- [ ] Protobuf-to-Misty converter
- [ ] Package Manager and Package Registry
- [ ] Formatter
- [ ] Linter

## Usage

This application uses [clap](https://crates.io/crates/clap) to provide a nice CLI interface for you.

There's ample documentation in the CLI itself, so you can run:

```sh
mistyc --help
```

You can also check our [documentation](https://github.com/thecodebreeze/misty/wiki) at GitHub for more information.

## License

This project is under the BSD-3 license.
