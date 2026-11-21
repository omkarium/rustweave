# Numbers functions in one script

The same integer is rendered in hexadecimal, binary, and arbitrary bases, then parsed back from each representation.

This script uses all 6 functions listed for `dw::core::Numbers`.

Import `dw::core::Numbers` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Numbers
output application/json
---
{
  toHex: [toHex(300), toHex(-300), toHex(0)],
  toBinary: [toBinary(6), toBinary(-6)],
  fromHex: [fromHex("AC"), fromHex("-ac")],
  fromBinary: fromBinary("1101"),
  fromRadix: [fromRadixNumber("q", 36), fromRadixNumber("101", 2)],
  toRadix: [toRadixNumber(26, 36), toRadixNumber(-300, 16)]
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/numbers.dwl
```

## Expected output

```json
{
  "toHex": [
    "12c",
    "-12c",
    "0"
  ],
  "toBinary": [
    "110",
    "-110"
  ],
  "fromHex": [
    172,
    -172
  ],
  "fromBinary": 13,
  "fromRadix": [
    26,
    5
  ],
  "toRadix": [
    "q",
    "-12c"
  ]
}
```

## Functions shown

`toHex`, `toBinary`, `fromHex`, `fromBinary`, `fromRadixNumber`, `toRadixNumber`
