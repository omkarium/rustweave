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
