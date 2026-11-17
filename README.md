# Rustweave

[![crates.io (binary)](https://img.shields.io/crates/v/rustweave.svg?label=rustweave)](https://crates.io/crates/rustweave)
[![crates.io (library)](https://img.shields.io/crates/v/rustweave-core.svg?label=rustweave-core)](https://crates.io/crates/rustweave-core)
[![docs.rs](https://docs.rs/rustweave-core/badge.svg)](https://docs.rs/rustweave-core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

`rustweave` is a Rust implementation of a DataWeave-compatible expression language. It provides an
expression engine, JSON input and output, exact decimal arithmetic, and a command-line interface.

The repository is a Cargo workspace:

- `rustweave-core` (`rustweave_core`): the parser, evaluator, runtime values, and standard library.
- `rustweave`: the `rustweave` command-line executable.

## Installation

```bash
cargo install rustweave
```

Or build from a checkout:

```bash
cargo build --release
./target/release/rustweave '1 + 1'
```

## Command line

```text
rustweave [run] [-i name=file]... (-f script.dwl | "<script>")
```

- A leading `run` is accepted and ignored.
- `-s` is accepted and ignored.
- `-i name=path` reads JSON from a file and binds it to `name`.
- `-f path` reads the script from a file.
- Any other argument is used as the script text.
- Errors print `[ERROR] message` to standard error and exit with status 1.

### Examples

Run an expression directly:

```bash
rustweave '[1, 2, 3] map $ * 2'
```

```json
[
  2,
  4,
  6
]
```

Bind a JSON file to a name and transform it. Given `payload.json`:

```json
{
  "orders": [
    { "id": 1, "customer": "ana", "total": 120.50 },
    { "id": 2, "customer": "ben", "total": 80 }
  ]
}
```

```bash
rustweave -i payload=payload.json \
  'payload.orders map { id: $.id, total: $.total }'
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

Put a full script in a file when the expression grows. Given `transform.dwl`:

```text
%dw 2.0
output application/json
---
payload.orders
  filter $.total > 100
  map { id: $.id, customer: $.customer }
```

```bash
rustweave -i payload=payload.json -f transform.dwl
```

```json
[
  {
    "id": 1,
    "customer": "ana"
  }
]
```

`rustweave run ...` is equivalent when your tooling always emits a leading `run`:

```bash
rustweave run -i payload=payload.json -f transform.dwl
```

On failure the command writes `[ERROR] message` to stderr and exits with status 1:

```console
$ rustweave 'notAFunction(1)'
[ERROR] unresolved name 'notAFunction'
$ echo $?
1
```

## Library API

Add the core crate to `Cargo.toml`:

```toml
[dependencies]
rustweave-core = "0.1"
```

```rust
fn main() -> Result<(), rustweave_core::DwError> {
    let output = rustweave_core::run(
        r#"output application/json
---
{ total: [1, 2, 3] map ($ * 2) }"#,
        &[],
    )?;
    println!("{output}");
    Ok(())
}
```

`rustweave_core::run(script, inputs)` takes a script and a list of named JSON texts. Each input
name becomes a variable in the script. The result is pretty-printed JSON text.

The crate also exposes:

- `rustweave_core::DwError` with a public `message` field, `Display`, and `std::error::Error`.
- `rustweave_core::Interp`, a reusable interpreter that can carry variables between evaluations.
- `rustweave_core::Value`, the runtime value type.
- `rustweave_core::json::parse` and `rustweave_core::json::write` for JSON input and output.

## Language surface

A script may start with a header containing `%dw 2.0`, `output application/json`,
`import * from ...`, named imports, `var`, `fun`, and annotations. If there is no header, the whole
text is the body. Expressions include:

- `null`, booleans, numbers, strings, arrays, and objects
- variable references, selections, safe selectors, indexing, and calls
- infix function calls
- `if`/`else`, `default`, `as`, `is`, and lambdas
- `$` and `$$` implicit lambda parameters
- `++`, `+`, `-`, `*`, `/`, `<<`, `--`, comparison operators, `and`, `or`, and `not`
- regex literals and array cons (`~`)
- dynamic object keys, string interpolation, and trailing commas
- ranges and range indexing
- `match`/`case` with literal, empty-array, and `[head ~ tail]` patterns, plus an optional `else`
  arm
- `do` blocks with one or more local variables and any body expression, including `if`/`else` and
  nested `do`

Infix function calls bind tighter than `++` and `default`, and looser than arithmetic, comparison,
`and`, `or`, `as`, and unary/postfix operators. A bare `$` or `$$` on the right of an infix call is
an implicit lambda over the tighter part of the expression.

Numbers use exact decimal arithmetic rather than binary floating point. Division keeps 34
significant digits. Numbers print without scientific notation or trailing zeros. Output JSON uses
two-space indentation, one element per line, and a space after each colon. Empty arrays and objects
are split across lines with a blank indented line between the brackets.

## Supported functions

### dw::Core

`map`, `filter`, `reduce`, `groupBy`, `sizeOf`, `isEmpty`, `sum`, `mapObject`, `pluck`,
`filterObject`, `flatMap`, `entriesOf`, `namesOf`, `keysOf`, `valuesOf`, `flatten`, `distinctBy`,
`orderBy`, `find`, `indexOf`, `lastIndexOf`, `contains`, `to`, `zip`, `unzip`, `upper`, `lower`,
`trim`, `startsWith`, `endsWith`, `splitBy`, `with`, `replace`, `joinBy`, `abs`, `ceil`, `floor`,
`round`, `pow`, `sqrt`, `mod`, `isInteger`, `isDecimal`, `isOdd`, `isEven`, `isBlank`, `minBy`,
`maxBy`, `max`, `min`, `avg`, `then`, `onNull`, `matches`, `match`, `scan`, `typeOf`, `random`,
`randomInt`, `uuid`, `log`

### dw::core::Arrays

`countBy`, `sumBy`, `some`, `every`, `take`, `drop`, `slice`, `indexOf`, `indexWhere`, `splitAt`,
`takeWhile`, `dropWhile`, `partition`, `firstWith`, `divideBy`, `splitWhere`, `join`, `leftJoin`,
`outerJoin`

### dw::core::Strings

`appendIfMissing`, `camelize`, `capitalize`, `charCode`, `charCodeAt`, `collapse`,
`countCharactersBy`, `countMatches`, `dasherize`, `everyCharacter`, `first`, `fromCharCode`,
`hammingDistance`, `isAlpha`, `isAlphanumeric`, `isLowerCase`, `isNumeric`, `isUpperCase`,
`isWhitespace`, `last`, `leftPad`, `levenshteinDistance`, `lines`, `mapString`, `ordinalize`,
`pluralize`, `prependIfMissing`, `remove`, `repeat`, `replaceAll`, `reverse`, `rightPad`,
`singularize`, `someCharacter`, `substring`, `substringAfter`, `substringAfterLast`,
`substringBefore`, `substringBeforeLast`, `substringBy`, `substringEvery`, `underscore`, `unwrap`,
`withMaxSize`, `words`, `wrapIfMissing`, `wrapWith`

### dw::core::Objects

`entrySet`, `nameSet`, `keySet`, `valueSet`, `everyEntry`, `someEntry`, `mergeWith`, `divideBy`,
`takeWhile`

### dw::core::Numbers

`toHex`, `toBinary`, `fromHex`, `fromBinary`, `fromRadixNumber`, `toRadixNumber`

## Error handling

Bad input, bad types, unresolved names, division by zero, invalid JSON, and unsupported syntax are
reported as `Result` errors rather than panics. The exact wording of an error message is not part
of the public contract.

## Scope

The current release targets JSON output and the expression and standard-library surface listed
above. XML, CSV, YAML, dates, binary, user-defined types, metadata, object spread, `update`, and
`using` are outside the current scope.

## Development

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent open-source project and is not
affiliated with, sponsored by, endorsed by, or approved by Salesforce, Inc., MuleSoft, LLC, or
their affiliates. The names are used only to describe compatibility with the DataWeave expression
language. No MuleSoft or Salesforce logos, icons, or other branding are used. See
[TRADEMARKS.md](TRADEMARKS.md).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this project shall be dual licensed as above, without any additional terms or
conditions. See [NOTICE](NOTICE) for attribution and [TRADEMARKS.md](TRADEMARKS.md) for trademark
information.
