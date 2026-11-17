# rustweave

`rustweave` is the command-line interface for the
[rustweave](https://github.com/omkarium/rustweave) expression engine, a Rust implementation of a
DataWeave-compatible expression language.

## Installation

```bash
cargo install rustweave
```

## Usage

```text
rustweave [run] [-i name=file]... (-f script.dwl | "<script>")
```

- A leading `run` is accepted and ignored.
- `-s` is accepted and ignored.
- `-i name=path` reads JSON from a file and binds it to `name`.
- `-f path` reads the script from a file.
- Any other argument is used as the script text.
- Errors print `[ERROR] message` to standard error and exit with status 1.

Examples:

```bash
rustweave '1 + 1'
rustweave -i payload=payload.json 'payload.orders map $.total'
rustweave -f transform.dwl
```

The engine is provided by the `rustweave-core` crate.

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not affiliated
with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names are used only to
describe compatibility with the DataWeave expression language. See `TRADEMARKS.md`.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at your option. See
`LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE`, and `TRADEMARKS.md`.
