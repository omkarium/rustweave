%dw 2.0
import * from dw::core::Objects
output application/json
---
do {
  var crate = {x: 4, y: 5, z: 6}
  var extra = {y: 9, w: 10}
  ---
  {
    entries: entrySet(crate),
    names: nameSet(crate),
    keys: keySet(crate),
    values: valueSet(crate),
    every: crate everyEntry ($ > 0),
    some: crate someEntry ($ == 6),
    merge: mergeWith(crate, extra),
    divide: divideBy(crate, 2),
    take: crate takeWhile ($ < 6)
  }
}
