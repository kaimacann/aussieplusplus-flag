# Aussie++

A Rust implementation of the Aussie++ language with a lexer, parser, resolver, and runtime.

## What it does

It parses and runs Aussie++ programs, including an alternate upside-down interpreter mode.

## Prerequisites, setup, usage, and operations

Install Rust with rustup. Run `cargo run --manifest-path aussieplusplus/Cargo.toml -- flag.aussie`; use `--upside-down` for the alternate mode.

### Common commands

```sh
cargo fmt --manifest-path aussieplusplus/Cargo.toml --check
cargo test --manifest-path aussieplusplus/Cargo.toml
cargo build --manifest-path aussieplusplus/Cargo.toml
```

## Security and Limitations

The interpreter runs supplied source code locally. Review programs before running untrusted input.

## Development and structure

Rust crate source is divided into lexer, parser, resolver, runtime, token, and upside-down modules.

## Validation

cargo fmt, cargo test, and cargo build use the manifest under `aussieplusplus/`.

| Check | Command |
| --- | --- |
| Format | `cargo fmt --manifest-path aussieplusplus/Cargo.toml --check` |
| Test | `cargo test --manifest-path aussieplusplus/Cargo.toml` |
| Build | `cargo build --manifest-path aussieplusplus/Cargo.toml` |

## References

Source references: [aussieplusplus/Cargo.toml](aussieplusplus/Cargo.toml).

## Contributing

No contribution guide was present in the inspected source snapshot.

## Credits and license

No repository license file or package license metadata was found in this snapshot.
