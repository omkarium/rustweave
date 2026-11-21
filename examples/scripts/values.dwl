%dw 2.0
import * from dw::util::Values
output application/json
---
{
  attr: attr(null, "lang"),
  field: field("sku"),
  index: index(2),
  updateField: {name: "Robin", age: 30} update "age" with ($ + 2),
  updatePath: {profile: {name: "Robin"}} update ["profile", field("name")] with "Rook",
  updateIndex: [4, 5, 6] update index(1) with 9,
  updateAll: [{role: "pilot"}, {role: "nav"}] update "role" with "Crew",
  mask: {name: "Robin", age: 30} mask "age" with 0
}
