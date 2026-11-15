# Aussie++ Flag

Aussie++ Flag lets you run two playful programs that draw the Australian flag for a university
challenge.

## Get started

Install Rust and Cargo. From the repository root, run either example:

```sh
cargo run --bin aussie_plus_plus -- samples/flag-right-side-up.aussie
cargo run --bin aussie_plus_plus -- --upside-down samples/flag.aussie
```

The interpreter prints a celebratory message after a sample executes. See the
[Aussie++ playground](https://aussieplusplus.vercel.app/) for the language.

## Validation

| Command                | Checks                        |
| ---------------------- | ----------------------------- |
| `pnpm run quality`     | Changed and untracked files   |
| `pnpm run quality:all` | Full configured Quality scope |
| `cargo test`           | Rust unit tests               |
| `pnpm run test`        | Project test suite            |
| `pnpm run build`       | Build output                  |

Quality can apply safe fixes; review the diff before committing. No separate browser or Playwright
test command is defined here.

Quality also runs Rust formatting, Clippy, and `cargo check`.

## Contributing

`src/` contains the interpreter and `samples/` contains the runnable examples. Open a pull request
against `development` and review the results of
[`repository-check.yml`](.github/workflows/repository-check.yml) before merging. Keep unrelated
changes out of the pull request.

## Documentation

- [Rust crate manifest](Cargo.toml)
- [Flag source](samples/flag.aussie)
- [Right-side-up example](samples/flag-right-side-up.aussie)

The sample output includes profanity. See [LICENSE](LICENSE) for the private-use rights notice.
