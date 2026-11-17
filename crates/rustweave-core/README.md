# rustweave-core

`rustweave-core` is the expression engine behind [rustweave](https://github.com/omkarium/rustweave).
It parses and evaluates a DataWeave-compatible expression language and returns JSON. The evaluator
is eager and supports exact decimal arithmetic, arrays, objects, lambdas, `do` blocks, `match`
expressions, regex literals, ranges, and a JSON-oriented standard library.

## Installation

```bash
cargo add rustweave-core
```

Or add it directly:

```toml
[dependencies]
rustweave-core = "0.1"
```

## Quick start

```rust
fn main() -> Result<(), rustweave_core::DwError> {
    let output = rustweave_core::run(
        "payload.orders map { id: $.id, total: $.total }",
        &[(
            "payload",
            r#"{"orders":[{"id":1,"total":120.50},{"id":2,"total":80}]}"#,
        )],
    )?;
    println!("{output}");
    Ok(())
}
```

```json
[
  {
    "id": 1,
    "total": 120.5
  },
  {
    "id": 2,
    "total": 80
  }
]
```

Inputs are named JSON texts. Each name becomes a variable in the script. The result is
pretty-printed JSON.

Arithmetic is exact decimal arithmetic, and division keeps 34 significant digits:

```rust
let output = rustweave_core::run("[0.1 + 0.2, 1 / 3]", &[]).unwrap();
assert!(output.starts_with("[\n  0.3,\n  0.3333"));
```

## Collections and the standard library

The language supports `map`, `filter`, `reduce`, `groupBy`, `orderBy`, joins, object transforms,
string and regex helpers, numeric helpers, and the `dw::Core`, `dw::core::Arrays`,
`dw::core::Strings`, `dw::core::Objects`, and `dw::core::Numbers` modules.

```rust
let output = rustweave_core::run(
    r#"%dw 2.0
import * from dw::core::Arrays
output application/json
---
{
  total: payload.orders sumBy $.total,
  paid: payload.orders filter $.paid map $.customer,
  byRegion: payload.orders groupBy $.region
}"#,
    &[(
        "payload",
        r#"{"orders":[
            {"customer":"ana","region":"north","total":120.5,"paid":true},
            {"customer":"ben","region":"south","total":80,"paid":false}
        ]}"#,
    )],
)
.unwrap();
println!("{output}");
```

```json
{
  "total": 200.5,
  "paid": [
    "ana"
  ],
  "byRegion": {
    "north": [
      {
        "customer": "ana",
        "region": "north",
        "total": 120.5,
        "paid": true
      }
    ],
    "south": [
      {
        "customer": "ben",
        "region": "south",
        "total": 80,
        "paid": false
      }
    ]
  }
}
```

## Expressions, `do`, and `match`

The engine supports `if`/`else`, `default`, `as`, `is`, lambdas, `$`/`$$`, safe selectors,
ranges, string interpolation, array cons, dynamic keys, trailing commas, `do` blocks, and
`match`/`case` with literal, empty-array, `[head ~ tail]`, and `else` arms.

```rust
let output = rustweave_core::run(
    r#"do {
         var status = "paid"
         ---
         { label: status match { case "paid" -> "Paid" else -> "Unknown" } }
       }"#,
    &[],
)
.unwrap();
assert_eq!(output, "{\n  \"label\": \"Paid\"\n}");
```

## Reusable interpreter and values

`Interp` keeps variables between evaluations, and `Value`, `Number`, and `json` are available for
embedding:

```rust
use rustweave_core::{Interp, Number, Value};

let interp = Interp::new().with_variable("x", Value::Number(Number::from_i64(41)));
assert_eq!(interp.run("x + 1", &[]).unwrap(), "42");
```

```rust
let value = rustweave_core::json::parse(r#"{"a":[1,2]}"#).unwrap();
assert_eq!(
    rustweave_core::json::write(&value),
    "{\n  \"a\": [\n    1,\n    2\n  ]\n}"
);
```

Errors are returned as `DwError`, not panics:

```rust
assert!(rustweave_core::run("notAFunction(1)", &[]).is_err());
```

## What is supported

- DataWeave expressions with `%dw 2.0` headers, imports, `var`, `fun`, overloads and defaults.
- JSON input and pretty-printed JSON output.
- Exact decimal numbers, arrays, objects, lambdas, `do`, `match`, regex literals and ranges.
- The `dw::Core`, `dw::core::Arrays`, `dw::core::Strings`, `dw::core::Objects` and
  `dw::core::Numbers` function surface.
- A reusable `Interp` and direct `Value`/`json` access.

See the repository README for the full function list and CLI examples.

## What is not supported yet

- XML, CSV, YAML, dates/times, binary values, user-defined types, metadata, `update`, and `using`.
- `read`, `write`, `readUrl`, `dw::Runtime` (`try`/`orElse`/`fail`), and Mule/Anypoint connectors.
- Lazy or streaming evaluation. This engine evaluates eagerly.
- Object spread syntax.
- The exact text of error messages is not part of the public contract, and regex behaviour follows
  Rust's `regex` crate rather than `java.util.regex`.

## Command line

The [rustweave](https://crates.io/crates/rustweave) crate provides the `rustweave` binary:

```text
rustweave [run] [-i name=file.json]... (-f script.dwl | "<script>")
```

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not affiliated
with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names are used only to
describe compatibility with the DataWeave expression language. See `TRADEMARKS.md`.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at your option. See
`LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE`, and `TRADEMARKS.md`.
