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

The engine is provided by the `rustweave-core` crate.

## Trademarks

DataWeave, MuleSoft, Anypoint, and Salesforce are trademarks or registered trademarks of
Salesforce, Inc. or its affiliates. `rustweave` is an independent project and is not affiliated
with, sponsored by, endorsed by, or approved by Salesforce or MuleSoft. The names are used only to
describe compatibility with the DataWeave expression language. See `TRADEMARKS.md`.

## License

Licensed under either the Apache License, Version 2.0 or the MIT license, at your option. See
`LICENSE-APACHE`, `LICENSE-MIT`, `NOTICE`, and `TRADEMARKS.md`.
