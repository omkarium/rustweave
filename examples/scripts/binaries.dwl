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
