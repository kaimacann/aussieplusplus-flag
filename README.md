# Aussie++ Flag

A UTS Cyber Security challenge project with two Aussie++ examples that draw the Australian flag and a Rust interpreter crate. The interpreter supports upside-down mode. Learn more about Aussie++ in the [playground documentation](https://aussieplusplus.vercel.app/).

## Getting started

Install Rust and Cargo. From the repository root, run the right-side-up example:

```sh
cargo run --manifest-path aussieplusplus/Cargo.toml --bin aussie_plus_plus -- flag-right-side-up.aussie
```

Run the upside-down example with:

```sh
cargo run --manifest-path aussieplusplus/Cargo.toml --bin aussie_plus_plus -- --upside-down flag.aussie
```

The interpreter prints a celebratory message after a sample executes.

## Development

From the repository root:

```sh
cargo fmt --manifest-path aussieplusplus/Cargo.toml
cargo test --manifest-path aussieplusplus/Cargo.toml
```

## Structure

`aussieplusplus/` contains the Rust crate and interpreter binary. `flag.aussie` and `flag-right-side-up.aussie` are runnable source examples.

## Limitations

The repository does not declare a license. The interpreter's sample output includes profanity.

## References

- [Rust crate manifest](aussieplusplus/Cargo.toml)
- [Flag source](flag.aussie)
- [Right-side-up source](flag-right-side-up.aussie)

## Contributing

Keep example changes runnable with the interpreter.
