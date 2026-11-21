# Coercions functions in one script

Text, temporal values, periods, binaries, and regular expressions are converted in both directions. Nullable variants show the miss behaviour next to the strict variants.

This script uses all 21 functions listed for `dw::util::Coercions`.

Import `dw::util::Coercions` to use the module functions. `toHex` is imported from Binaries for the binary sample.

## Script

```dataweave
%dw 2.0
import * from dw::util::Coercions
import toHex from dw::core::Binaries
output application/json
---
{
  toArray: toArray("river"),
  toBinary: toHex(toBinary("RX", "UTF-8")),
  toBoolean: [toBoolean("true"), toBoolean("FALSE")],
  toDate: toDate("2019-04-23"),
  toDateOrNull: toDateOrNull("bad"),
  toDateTime: toDateTime("2019-04-23T08:15:30Z"),
  toDateTimeOrNull: toDateTimeOrNull("bad"),
  toLocalDateTime: toLocalDateTime("2019-04-23T08:15:30"),
  toLocalDateTimeOrNull: toLocalDateTimeOrNull("bad"),
  toLocalTime: toLocalTime("08:15:30.250", "HH:mm:ss.n"),
  toLocalTimeOrNull: toLocalTimeOrNull("bad"),
  toNumber: [toNumber("2.0"), toNumber(|PT2H5M|, "seconds")],
  toNumberOrNull: toNumberOrNull("bad"),
  toPeriod: toPeriod("PT2H5M"),
  toRegex: toString(toRegex("a-Z")),
  toString: [
    toString(2.0),
    toString(|2019-04-23|, "uuuu/MM/dd"),
    toString(toBinary("RX", "UTF-8"), "UTF-8")
  ],
  toTime: toTime("14:15:30-05:00"),
  toTimeOrNull: toTimeOrNull("bad"),
  toTimeZone: toTimeZone("-05:00"),
  toURI: toURI("https://example.test/a b"),
  toUri: toUri("https://example.test/c d")
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/coercions.dwl
```

## Expected output

```json
{
  "toArray": [
    "r",
    "i",
    "v",
    "e",
    "r"
  ],
  "toBinary": "5258",
  "toBoolean": [
    true,
    false
  ],
  "toDate": "2019-04-23",
  "toDateOrNull": null,
  "toDateTime": "2019-04-23T08:15:30Z",
  "toDateTimeOrNull": null,
  "toLocalDateTime": "2019-04-23T08:15:30",
  "toLocalDateTimeOrNull": null,
  "toLocalTime": "08:15:30.250",
  "toLocalTimeOrNull": null,
  "toNumber": [
    2,
    7500
  ],
  "toNumberOrNull": null,
  "toPeriod": "PT2H5M",
  "toRegex": "a-Z",
  "toString": [
    "2",
    "2019/04/23",
    "RX"
  ],
  "toTime": "14:15:30-05:00",
  "toTimeOrNull": null,
  "toTimeZone": "-05:00",
  "toURI": "https://example.test/a b",
  "toUri": "https://example.test/c d"
}
```

## Functions shown

`toArray`, `toBinary`, `toBoolean`, `toDate`, `toDateOrNull`, `toDateTime`, `toDateTimeOrNull`, `toLocalDateTime`, `toLocalDateTimeOrNull`, `toLocalTime`, `toLocalTimeOrNull`, `toNumber`, `toNumberOrNull`, `toPeriod`, `toRegex`, `toString`, `toTime`, `toTimeOrNull`, `toTimeZone`, `toURI`, `toUri`
