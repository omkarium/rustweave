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

## Exit status

- `0` on success; the JSON result is printed to stdout.
- `1` on an input, parse, or evaluation error; `[ERROR] message` is printed to stderr.

## Scope

The CLI targets JSON output. The parser, evaluator, exact decimal arithmetic, standard library, and
supported/unsupported feature list are provided by the `rustweave-core` crate; see its
documentation. XML, YAML, CSV, Multipart, Crypto, data formats, and timers are not supported yet.
`dw::Runtime` is out of scope, and `using` is not supported.

The engine is provided by the `rustweave-core` crate.

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not affiliated
with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names are used only to
describe compatibility with the DataWeave expression language. See `TRADEMARKS.md`.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at your option. See
`LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE`, and `TRADEMARKS.md`.
