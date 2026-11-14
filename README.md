# Aussie++ Flag

## Quality

This repository contains Rust source only. Standards Quality 2.2.0 has no built-in Rust profile, so `quality.json` leaves the JavaScript and TypeScript source extensions empty and runs Rust formatting, Clippy, and `cargo check` directly. Build and functional test commands remain separate.

A UTS Cyber Security challenge project with two Aussie++ examples that draw the Australian flag and a Rust interpreter crate. The interpreter supports upside-down mode. Learn more about Aussie++ in the [playground documentation](https://aussieplusplus.vercel.app/).

## Getting started

Install Rust and Cargo. From the repository root, run the right-side-up example:

```sh
cargo run --manifest-path Cargo.toml --bin aussie_plus_plus -- samples/flag-right-side-up.aussie
```

Run the upside-down example with:

```sh
cargo run --manifest-path Cargo.toml --bin aussie_plus_plus -- --upside-down samples/flag.aussie
```

The interpreter prints a celebratory message after a sample executes.

## Development

From the repository root:

```sh
cargo fmt --manifest-path Cargo.toml
cargo test --manifest-path Cargo.toml
```

Install Node.js and pnpm 10.30.3, then install the pinned Terra Quality CLI and run its full Rust workflow:

```sh
pnpm install
pnpm run quality:all
```

## Structure

`src/` contains the Rust interpreter. `samples/` contains the runnable flag examples.

## Limitations

The repository includes a private-use rights notice. The interpreter's sample output includes profanity.

## References

- [Rust crate manifest](Cargo.toml)
- [Flag source](samples/flag.aussie)
- [Right-side-up source](samples/flag-right-side-up.aussie)

## Contributing

Keep example changes runnable with the interpreter.
