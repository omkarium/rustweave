# Binaries functions in one script

A binary literal is converted to hex and base64, parsed back, concatenated, and read or written as UTF-8 lines.

This script uses all 7 functions listed for `dw::core::Binaries`.

Import `dw::core::Binaries` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Binaries
output application/json
---
{
  fromHex: toHex(fromHex("4E6F727468")),
  toHex: toHex(|4E6F727468|),
  toBase64: toBase64(fromHex("4E6F727468")),
  fromBase64: toHex(fromBase64("Tm9ydGg=")),
  concatWith: toHex(fromHex("CAFE") concatWith fromHex("BEEF")),
  readLinesWith: fromHex("4E6F7274680A536F7574680A") readLinesWith "UTF-8",
  writeLinesWith: toHex(writeLinesWith(["North", "South"], "UTF-8"))
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/binaries.dwl
```

## Expected output

```json
{
  "fromHex": "4E6F727468",
  "toHex": "4E6F727468",
  "toBase64": "Tm9ydGg=",
  "fromBase64": "4E6F727468",
  "concatWith": "CAFEBEEF",
  "readLinesWith": [
    "North",
    "South"
  ],
  "writeLinesWith": "4E6F7274680A536F7574680A"
}
```

## Functions shown

`concatWith`, `fromBase64`, `fromHex`, `readLinesWith`, `toBase64`, `toHex`, `writeLinesWith`
