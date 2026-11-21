# Examples

The module guides below put a complete DataWeave script and its expected output side by side. Each
one uses every function listed for that module, so the examples show how the functions combine
rather than isolating one call at a time. Together they cover all 345 namespace/name entries in the
function index.

Build once from the repository root:

```bash
cargo build -p rustweave
```

Then run any script with `./target/debug/rustweave`, or use `cargo run -q -p rustweave --` if you
prefer to build on demand. The runnable copies live in [`scripts/`](scripts/).

## Module guides

| Module | Functions | Guide |
| --- | ---: | --- |
| `dw::Core` | 74 | [core.md](core.md) |
| `dw::core::Arrays` | 19 | [arrays.md](arrays.md) |
| `dw::core::Strings` | 47 | [strings.md](strings.md) |
| `dw::core::Objects` | 9 | [objects.md](objects.md) |
| `dw::core::Numbers` | 6 | [numbers.md](numbers.md) |
| `dw::core::Dates` | 13 | [dates.md](dates.md) |
| `dw::core::Periods` | 9 | [periods.md](periods.md) |
| `dw::core::Binaries` | 7 | [binaries.md](binaries.md) |
| `dw::core::URL` | 6 | [url.md](url.md) |
| `dw::core::Types` | 37 | [types.md](types.md) |
| `dw::util::Tree` | 9 | [tree.md](tree.md) |
| `dw::util::Coercions` | 21 | [coercions.md](coercions.md) |
| `dw::util::Values` | 5 | [values.md](values.md) |
| `dw::util::Math` | 17 | [math.md](math.md) |
| `dw::util::Diff` | 1 | [diff.md](diff.md) |
| `dw::Crypto` | 5 | [crypto.md](crypto.md) |
| `dw::util::Timer` | 4 | [timer.md](timer.md) |
| `dw::extra::Common` | 56 | [extra-common.md](extra-common.md) |

Each guide contains the full script, the command to run it, the expected JSON output, and the list
of functions shown.

## Language and mixed-module guides

[language.md](language.md) shows `using`, object head-tail spread, and value metadata.
[types-and-namespaces.md](types-and-namespaces.md) shows declared types, namespace prefixes, and
output format IDs.
[combined.md](combined.md) combines Core, Arrays, Objects, and Tree in one inventory report.
[scenarios.md](scenarios.md) contains three larger business-style pipelines with input files from
[`data/`](data/).

## Notes

- The extension crypto helpers have their own guide: [extension-crypto.md](extension-crypto.md).
- Run the scripts from the repository root. `core.md` uses `readUrl` with the local path
  `examples/data/orders.json`, and the scenarios use the JSON files under `data/`.
- Core logging helpers write to standard error and return their value. The expected output shows
  the returned values.
- `random`, `randomInt`, `uuid`, `now`, `today`, `tomorrow`, and `yesterday` are shown through
  stable checks such as type names, ranges, and day differences rather than fixed values.
- Output uses the current JSON writer: two-space indentation, one element per line, and no trailing
  zeros in numbers.
