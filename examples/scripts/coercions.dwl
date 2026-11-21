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
