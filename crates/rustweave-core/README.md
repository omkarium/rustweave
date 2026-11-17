# rustweave-core

`rustweave-core` is the expression engine behind [rustweave](https://github.com/omkarium/rustweave).
It parses and evaluates a DataWeave-compatible expression language with exact decimal arithmetic,
JSON input and output, arrays, objects, lambdas, `do` blocks, `match` expressions, and a
JSON-oriented standard library.

## Library usage

```toml
[dependencies]
rustweave-core = "0.1"
```

```rust
fn main() -> Result<(), rustweave_core::DwError> {
    let output = rustweave_core::run("1 + 1", &[])?;
    println!("{output}");
    Ok(())
}
```

Inputs are named JSON texts that become variables in the script:

```rust
let output = rustweave_core::run(
    "payload.value * 2",
    &[("payload", r#"{"value": 21}"#)],
)?;
assert_eq!(output, "42");
```

The crate also exposes `rustweave_core::Interp` for reusable evaluation, `rustweave_core::Value`
for runtime values, and `rustweave_core::json` for parsing and writing JSON.

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not affiliated
with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names are used only to
describe compatibility with the DataWeave expression language. See `TRADEMARKS.md`.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at your option. See
`LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE`, and `TRADEMARKS.md`.
