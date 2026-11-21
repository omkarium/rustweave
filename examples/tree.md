# Tree functions in one script

A nested inventory is mapped leaf by leaf, filtered at the array, object, and whole-tree levels, searched by value and node name, and described by path helpers.

This script uses all 9 functions listed for `dw::util::Tree`.

Import `dw::util::Tree` to use the module functions. The path constants come from the same module.

## Script

```dataweave
%dw 2.0
import * from dw::util::Tree
output application/json
---
do {
  var inventory = {
    shelf: [
      {name: "Tea", code: "T-1", tags: [{name: "green"}, {name: "loose"}]},
      {name: "Rope", code: "R-2", tags: [{name: "nylon"}]}
    ]
  }
  var nodePath = [
    {kind: OBJECT_TYPE, selector: "shelf", namespace: null},
    {kind: ARRAY_TYPE, selector: 0, namespace: null},
    {kind: OBJECT_TYPE, selector: "code", namespace: null}
  ]
  ---
  {
    pathChecks: [
      isArrayType(nodePath),
      isAttributeType([{kind: ATTRIBUTE_TYPE, selector: "amount", namespace: null}]),
      isObjectType(nodePath)
    ],
    expression: asExpressionString([
      {kind: OBJECT_TYPE, selector: "invoice", namespace: null},
      {kind: ATTRIBUTE_TYPE, selector: "amount", namespace: null}
    ]),
    mapped: inventory mapLeafValues (value, path) ->
      if (value is String) upper(value) else value,
    filteredArray: [3, {name: ["", false], count: 17}, "42", null]
      filterArrayLeafs ((value, path) -> !(value is Null or value is String)),
    filteredObject: {name: "Rook", lastName: null, age: 45}
      filterObjectLeafs ((value, path) -> !(value is Null or value is String)),
    filteredTree: inventory
      filterTree ((value, path) -> if (value is String) !isEmpty(value) else true),
    found: inventory nodeExists ((value, path) ->
      path[-1].selector == "name" and value == "green"),
    missing: inventory nodeExists ((value, path) ->
      path[-1].selector == "name" and value == "Coffee")
  }
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/tree.dwl
```

## Expected output

```json
{
  "pathChecks": [
    false,
    true,
    true
  ],
  "expression": ".invoice.@amount",
  "mapped": {
    "shelf": [
      {
        "name": "TEA",
        "code": "T-1",
        "tags": [
          {
            "name": "GREEN"
          },
          {
            "name": "LOOSE"
          }
        ]
      },
      {
        "name": "ROPE",
        "code": "R-2",
        "tags": [
          {
            "name": "NYLON"
          }
        ]
      }
    ]
  },
  "filteredArray": [
    3,
    {
      "name": [
        false
      ],
      "count": 17
    }
  ],
  "filteredObject": {
    "age": 45
  },
  "filteredTree": {
    "shelf": [
      {
        "name": "Tea",
        "code": "T-1",
        "tags": [
          {
            "name": "green"
          },
          {
            "name": "loose"
          }
        ]
      },
      {
        "name": "Rope",
        "code": "R-2",
        "tags": [
          {
            "name": "nylon"
          }
        ]
      }
    ]
  },
  "found": true,
  "missing": false
}
```

## Functions shown

`asExpressionString`, `filterArrayLeafs`, `filterObjectLeafs`, `filterTree`, `isArrayType`, `isAttributeType`, `isObjectType`, `mapLeafValues`, `nodeExists`
