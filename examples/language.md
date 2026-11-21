# Language completeness in one script

This guide shows `using` bindings, object head-tail spread, and value metadata together.

## Script

```dataweave
%dw 2.0
output application/json
var tag = "tree"
---
{
  usingBasic: using (x = 3) x + 1,
  usingChain: using (a = 2, b = a * 4) [a, b],
  spread: {id: 1 ~ {name: "oak"}},
  dynamicSpread: { (tag): true ~ {height: 12} },
  metaValue: (5 as Number {unit: "cm"}).^unit,
  metaAll: (1 as Number {a: 1, b: 2}).^,
  metaMissing: (1).^x
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/language.dwl
```

## Expected output

```json
{
  "usingBasic": 4,
  "usingChain": [
    2,
    8
  ],
  "spread": {
    "id": 1,
    "name": "oak"
  },
  "dynamicSpread": {
    "tree": true,
    "height": 12
  },
  "metaValue": "cm",
  "metaAll": {
    "a": 1,
    "b": 2
  },
  "metaMissing": null
}
```

## Features shown

`using`, object head-tail spread, dynamic head keys, `value as Type { ... }`, `value.^key`, and
`value.^`
