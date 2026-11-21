%dw 2.0
import diff from dw::util::Diff
output application/json
---
{
  missing: diff({x: 2}, {y: 2}),
  nested: diff([{x: 2}], [{x: 3}]),
  equal: diff({x: 2}, {x: 2}),
  unordered: diff({x: 1, y: 2}, {y: 2, x: 1}, {unordered: true})
}
