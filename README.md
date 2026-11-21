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

Bind a JSON file to a name and transform it. Given `readings.json`:

```json
{
  "devices": [
    { "code": "d1", "zone": "north", "reading": 21.50 },
    { "code": "d2", "zone": "south", "reading": 18 }
  ]
}
```

```bash
rustweave -i feed=readings.json \
  'feed.devices map { code: $.code, reading: $.reading }'
```

```json
[
  {
    "code": "d1",
    "reading": 21.5
  },
  {
    "code": "d2",
    "reading": 18
  }
]
```

Put a full script in a file when the expression grows. Given `transform.dwl`:

```text
%dw 2.0
output application/json
---
feed.devices
  filter $.reading > 20
  map { code: $.code, zone: $.zone }
```

```bash
rustweave -i feed=readings.json -f transform.dwl
```

```json
[
  {
    "code": "d1",
    "zone": "north"
  }
]
```

`rustweave run ...` is equivalent when your tooling always emits a leading `run`:

```bash
rustweave run -i feed=readings.json -f transform.dwl
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
rustweave-core = "0.3"
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
name becomes a variable in the script. The default result is pretty-printed JSON text.

The crate also exposes:

- `rustweave_core::DwError` with a public `message` field, `Display`, and `std::error::Error`.
- `rustweave_core::Interp`, a reusable interpreter that can carry variables between evaluations.
- `rustweave_core::Value`, the runtime value type.
- `rustweave_core::json::parse` and `rustweave_core::json::write` for JSON input and output.

## Function index

The `rustweave_core::functions` module provides a typed index of every supported function,
including its DataWeave namespace and a short description. The complete list is available as
[`Function::ALL`](https://docs.rs/rustweave-core/latest/rustweave_core/functions/struct.Function.html#associatedconstant.ALL).

## Language surface

A script may start with a header containing `%dw 2.0`, `output application/json`,
`import * from ...`, named imports, `var`, `fun`, `type` declarations, and annotations. If there is
no header, the whole text is the body. Expressions include:

- `null`, booleans, numbers, strings, arrays, and objects
- variable references, single and safe selections, multi-value `.*key`, descendant `..key`,
  key-value `.&key`, XML attribute `.@key`/`.@`, filter `[?(predicate)]`, indexing, postfix
  `!` assertions, and calls
- infix function calls
- `if`/`else`, `default`, `as`, `is`, and lambdas
- `$` and `$$` implicit lambda parameters
- `++`, `+`, `-`, `*`, `/`, `<<`, `>>`, `--`, `==`, coercing `~=`, comparison operators,
  `and`, `or`, `not`, and prefix `!`
- regex literals and array cons (`~`)
- dynamic object keys, string interpolation, and trailing commas
- ranges and range indexing
- date, time, date-time, local date-time, local time, period, and binary values and literals
- user-defined `type` declarations
- `update` expressions in the form `value update selector with newValue`
- `match`/`case` with type, named-literal, guard, regex-capture, `[head ~ tail]`, `[]`,
  `{key: value ~ tail}`, and `{}` patterns, plus an optional `else` arm
- `do` blocks with one or more local variables and any body expression, including `if`/`else` and
  nested `do`
- `using (name = expression, ...) body` prefix bindings
- object head-tail spread with `{ key: value ~ tail }`, including dynamic head keys
- value metadata with `value as Type { ... }`, `value.^key`, and `value.^`

Infix function calls bind tighter than `++` and `default`, and looser than arithmetic, comparison,
`and`, `or`, `as`, and unary/postfix operators. `not` is a low-precedence prefix operator that
applies to the whole following expression, while `!` binds tightly. A bare `$` or `$$` on the right of an infix call is
an implicit lambda over the tighter part of the expression.

Numbers use exact decimal arithmetic rather than binary floating point. Division keeps 34
significant digits. Numbers print without scientific notation or trailing zeros. Output JSON uses
two-space indentation, one element per line, and a space after each colon. Empty arrays and objects
are split across lines with a blank indented line between the brackets.

## Standard library examples

Create and manipulate dates and periods:

```text
%dw 2.0
import * from dw::core::Dates
import * from dw::core::Periods
output application/json
---
{
  day: date({year: 2021, month: 2, day: 14}),
  startOfMonth: atBeginningOfMonth(|2021-02-14|),
  tomorrow: |2021-02-14T11:30:00Z| + days(1),
  leap: isLeapYear(|2024-01-01|)
}
```

```json
{
  "day": "2021-02-14",
  "startOfMonth": "2021-02-01",
  "tomorrow": "2021-02-15T11:30:00Z",
  "leap": true
}
```

Transform every leaf in a tree:

```text
%dw 2.0
import * from dw::util::Tree
output application/json
---
{ inventory: { title: "tools", labels: ["steel", "wood"] } } mapLeafValues (value, path) ->
  if (value is String) upper(value) else value
```

```json
{
  "inventory": {
    "title": "TOOLS",
    "labels": [
      "STEEL",
      "WOOD"
    ]
  }
}
```

Read a JSON string and write a compact JSON string:

```text
%dw 2.0
output application/json
---
{
  parsed: read("{\"sku\":\"g-17\",\"count\":4}", "application/json"),
  encoded: write({sku: "g-17", count: 4}, "application/json", {indent: false})
}
```

```json
{
  "parsed": {
    "sku": "g-17",
    "count": 4
  },
  "encoded": "{\"sku\": \"g-17\",\"count\": 4}"
}
```

Work with binary values:

```text
%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  hex: toHex(fromHex("54696465")),
  base64: toBase64(fromHex("54696465")),
  literalHex: toHex(|54696465|)
}
```

```json
{
  "hex": "54696465",
  "base64": "VGlkZQ==",
  "literalHex": "54696465"
}
```

## Extension module

`dw::extra::Common` adds general-purpose helpers that are outside the DataWeave compatibility
surface. Import it explicitly; its names do not replace the core modules.

```text
%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  chunks: chunk(range(1, 8), 3),
  picked: pick({code: "g-17", count: 4, note: null}, ["code", "count"]),
  fixed: toFixed(2.345, 2),
  titled: title("quiet HARBOR"),
  piped: pipe([(n) -> n + 2, (n) -> n * 3])(4)
}
```

```json
{
  "chunks": [
    [
      1,
      2,
      3
    ],
    [
      4,
      5,
      6
    ],
    [
      7
    ]
  ],
  "picked": {
    "code": "g-17",
    "count": 4
  },
  "fixed": "2.35",
  "titled": "Quiet Harbor",
  "piped": 18
}
```

The collection, string, and null helpers can be combined with the earlier helpers:

```text
%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  folded: fold([3, 4, 5], 0, (item, acc) -> acc + item),
  windows: windows([1, 2, 3, 4], 2),
  split: splitOnce("alpha/beta", "/"),
  fallback: unwrapOr(null, "ready")
}
```

```json
{
  "folded": 12,
  "windows": [
    [
      1,
      2
    ],
    [
      2,
      3
    ],
    [
      3,
      4
    ]
  ],
  "split": [
    "alpha",
    "beta"
  ],
  "fallback": "ready"
}
```

Module-by-module examples with full scripts, commands, and expected output are in
[`examples/`](examples/), including combined scenarios that use several modules at once.

## Structured formats

CSV, XML, and YAML are available through `read`, `write`, `readUrl`, and the `output` header.
MIME matching ignores case and trims parameters such as `; charset=UTF-8`:

- CSV: `application/csv`, `text/csv`
- XML: `application/xml`, `text/xml`
- YAML: `application/yaml`, `application/x-yaml`, `text/yaml`

CSV reader and writer options include `header`, `separator`, `quote`, `escape`,
`recordSeparator`, `skipEmptyLine`, and `nullValue`. XML options include `attributePrefix`,
`textKey`, `namespaceSeparator`, `writeDeclaration`, and `indent`. YAML options include `indent`
and `writeDocumentStart`. The same options can be supplied in the `output` header.

Read quoted CSV fields and write XML and YAML values:

```text
%dw 2.0
output application/json
---
{
  rows: read("code,note\nG-17,\"calm, grey\"\n", "application/csv"),
  xml: write({fleet: {"@id": "f1", name: "North Star", part: ["hull", "sail"]}}, "application/xml"),
  yaml: write({ship: {name: "Kite", tags: ["calm", "sea"]}}, "application/yaml")
}
```

Write CSV directly with header properties:

```text
%dw 2.0
output application/csv header=false, separator=";"
---
[{code: "G-17", note: "calm, grey"}, {code: "G-18", note: "dry dock"}]
```

```text
G-17;calm, grey
G-18;dry dock
```

Write an indented XML document:

```text
%dw 2.0
output application/xml indent=true
---
{fleet: {"@id": "f1", name: "North Star", part: ["hull", "sail"]}}
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<fleet id="f1">
  <name>North Star</name>
  <part>hull</part>
  <part>sail</part>
</fleet>
```

Write a YAML document:

```text
%dw 2.0
output application/yaml
---
{ship: {name: "Kite", tags: ["calm", "sea"]}, empty: null}
```

```yaml
ship:
  name: Kite
  tags:
    - calm
    - sea
empty: null
```

`readUrl` selects a format from `.json`, `.csv`, `.xml`, `.yaml`, `.yml`, and falls back to
`text/plain` for other extensions.

## Supported functions

### dw::Core

`map`, `filter`, `reduce`, `groupBy`, `sizeOf`, `isEmpty`, `sum`, `mapObject`, `pluck`,
`filterObject`, `flatMap`, `entriesOf`, `namesOf`, `keysOf`, `valuesOf`, `flatten`, `distinctBy`,
`orderBy`, `find`, `indexOf`, `lastIndexOf`, `contains`, `to`, `zip`, `unzip`, `upper`, `lower`,
`trim`, `startsWith`, `endsWith`, `splitBy`, `with`, `replace`, `joinBy`, `abs`, `ceil`, `floor`,
`round`, `pow`, `sqrt`, `mod`, `isInteger`, `isDecimal`, `isOdd`, `isEven`, `isBlank`, `minBy`,
`maxBy`, `max`, `min`, `avg`, `then`, `onNull`, `matches`, `match`, `scan`, `typeOf`, `random`,
`randomInt`, `uuid`, `log`, `daysBetween`, `isLeapYear`, `now`, `read`, `write`, `readUrl`,
`logDebug`, `logInfo`, `logWarn`, `logError`, `logWith`, `plusPlus`, `minusMinus`

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

### dw::core::Dates

`atBeginningOfDay`, `atBeginningOfHour`, `atBeginningOfMonth`, `atBeginningOfWeek`,
`atBeginningOfYear`, `date`, `dateTime`, `localDateTime`, `localTime`, `time`, `today`, `tomorrow`,
`yesterday`

### dw::core::Periods

`between`, `days`, `duration`, `hours`, `minutes`, `months`, `period`, `seconds`, `years`

### dw::core::Binaries

`concatWith`, `fromBase64`, `fromHex`, `readLinesWith`, `toBase64`, `toHex`, `writeLinesWith`

### dw::core::URL

`compose`, `decodeURI`, `decodeURIComponent`, `encodeURI`, `encodeURIComponent`, `parseURI`

### dw::core::Types

`arrayItem`, `baseTypeOf`, `functionParamTypes`, `functionReturnType`, `intersectionItems`,
`isAnyType`, `isArrayType`, `isBinaryType`, `isBooleanType`, `isDateTimeType`, `isDateType`,
`isFunctionType`, `isIntersectionType`, `isKeyType`, `isLiteralType`, `isLocalDateTimeType`,
`isLocalTimeType`, `isNamespaceType`, `isNothingType`, `isNullType`, `isNumberType`, `isObjectType`,
`isPeriodType`, `isRangeType`, `isReferenceType`, `isRegexType`, `isStringType`, `isTimeType`,
`isTimeZoneType`, `isTypeType`, `isUnionType`, `isUriType`, `literalValueOf`, `metadataOf`,
`nameOf`, `objectFields`, `unionItems`

### dw::util::Tree

`asExpressionString`, `filterArrayLeafs`, `filterObjectLeafs`, `filterTree`, `isArrayType`,
`isAttributeType`, `isObjectType`, `mapLeafValues`, `nodeExists`

### dw::util::Coercions

`toArray`, `toBinary`, `toBoolean`, `toDate`, `toDateOrNull`, `toDateTime`, `toDateTimeOrNull`,
`toLocalDateTime`, `toLocalDateTimeOrNull`, `toLocalTime`, `toLocalTimeOrNull`, `toNumber`,
`toNumberOrNull`, `toPeriod`, `toRegex`, `toString`, `toTime`, `toTimeOrNull`, `toTimeZone`,
`toURI`

### dw::util::Values

`attr`, `field`, `index`, `mask`, `update`

### dw::util::Math

`acos`, `asin`, `atan`, `cos`, `decimalAdd`, `decimalDivide`, `decimalMultiply`, `decimalPow`,
`decimalRound`, `decimalSqrt`, `decimalSubtract`, `log10`, `logn`, `sin`, `tan`, `toDegrees`,
`toRadians`

### dw::util::Diff

`diff`

### dw::Crypto

`MD5`, `SHA1`, `hashWith`, `HMACWith`, `HMACBinary`

### dw::util::Timer

`currentMilliseconds`, `duration`, `time`, `toMilliseconds`

### dw::extra::Common

`enumerate`, `range`, `sorted`, `reversed`, `chunk`, `compact`, `unique`, `fromEntries`, `assign`,
`pick`, `omit`, `hasOwn`, `parseInt`, `parseFloat`, `toFixed`, `clamp`, `trunc`, `sign`, `title`,
`lstrip`, `rstrip`, `zfill`, `partition`, `rpartition`, `identity`, `pipe`, `compose`, `fold`,
`filterMap`, `stepBy`, `position`, `rposition`, `dedup`, `windows`, `chunksExact`, `binarySearch`,
`rotateLeft`, `rotateRight`, `swap`, `fill`, `truncate`, `splitOnce`, `rsplitOnce`, `stripPrefix`,
`stripSuffix`, `unwrapOr`, `unwrapOrElse`, `mapOr`, `andThen`, `orElse`, `aesGcmEncrypt`,
`aesGcmDecrypt`, `randomBytes`, `hkdfSha256`, `pbkdf2Sha256`, `constantTimeEquals`

## Error handling

Bad input, bad types, unresolved names, division by zero, invalid JSON, and unsupported syntax are
reported as `Result` errors rather than panics. The exact wording of an error message is not part
of the public contract.

## Scope

The current release targets JSON, CSV, XML, and YAML output and the expression and
standard-library surface listed above, including date/time, period, binary, type, tree, coercion,
value, math, URL, diff, crypto, and timer helpers. Multipart and other data formats are outside
the current scope. `dw::Runtime` is out of scope.

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
