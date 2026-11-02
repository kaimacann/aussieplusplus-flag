# Aussie++ Flag

Two Aussie++ source examples draw the Australian flag, and a Rust interpreter crate runs the examples. The sample currently prints a celebratory message after execution.

## Getting started

Install Rust and Cargo. From `aussieplusplus/`, run:

```sh
cargo run -- ../flag.aussie
```

Cargo builds and runs the interpreter; it prints `CHEERS C***!` after this sample executes.

### Commands

From `aussieplusplus/`, run `cargo fmt` and `cargo test`.

## Security and Limitations

The repository contains example source and an interpreter; it does not declare a license. The sample's output includes profanity.

## Structure

`aussieplusplus/` is the Rust crate and binary. `flag.aussie` and `flag-right-side-up.aussie` are runnable source examples.

## References

- [Rust crate manifest](aussieplusplus/Cargo.toml)
- [Flag source](flag.aussie)
- [Right-side-up source](flag-right-side-up.aussie)

## Contributing

Keep example changes runnable with the interpreter.

## Credits and license

No repository license file or crate license declaration is present. Ask the maintainer about reuse terms.
