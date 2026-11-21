# Values functions in one script

Field, index, and attribute selectors are created, then used in nested updates and a mask operation.

This script uses all 5 functions listed for `dw::util::Values`.

Import `dw::util::Values` to use the module functions.

## Script

```dataweave
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
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/values.dwl
```

## Expected output

```json
{
  "attr": {
    "kind": "Attribute",
    "namespace": null,
    "selector": "lang"
  },
  "field": {
    "kind": "Object",
    "namespace": null,
    "selector": "sku"
  },
  "index": {
    "kind": "Array",
    "namespace": null,
    "selector": 2
  },
  "updateField": {
    "name": "Robin",
    "age": 32
  },
  "updatePath": {
    "profile": {
      "name": "Rook"
    }
  },
  "updateIndex": [
    4,
    9,
    6
  ],
  "updateAll": [
    {
      "role": "Crew"
    },
    {
      "role": "Crew"
    }
  ],
  "mask": {
    "name": "Robin",
    "age": 0
  }
}
```

## Functions shown

`attr`, `field`, `index`, `mask`, `update`
