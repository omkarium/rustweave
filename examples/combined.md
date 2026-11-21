# Core, Arrays, Objects, and Tree together

This script combines four namespaces in one inventory report. Core transformations and aggregation feed Arrays helpers, Objects entry helpers decorate the grouped result, and Tree helpers map and filter the nested warehouse structure.

```dataweave
%dw 2.0
import * from dw::core::Arrays
import everyEntry, someEntry, mergeWith from dw::core::Objects
import * from dw::util::Tree
output application/json
---
do {
  var stock = [
    {sku: "r1", dept: "tools", qty: 7, open: true},
    {sku: "r2", dept: "tools", qty: 2, open: false},
    {sku: "r3", dept: "kitchen", qty: 11, open: true},
    {sku: "r4", dept: "kitchen", qty: 4, open: true}
  ]
  var byDept = stock groupBy $.dept
  var deptTotals = byDept mapObject { ($$): sum($ map $.qty) }
  var tree = {
    warehouse: {
      zones: [
        {name: "north", stock: stock},
        {name: "south", stock: []}
      ]
    }
  }
  ---
  {
    openSkus: stock filter $.open map $.sku,
    totalQty: stock sumBy $.qty,
    openCount: stock countBy $.open,
    allPositive: stock every ($.qty > 0),
    anyLow: stock some ($.qty < 5),
    firstLow: (stock firstWith ($.qty < 5)).sku,
    split: do {
      var parts = stock splitAt 2
      ---
      {left: parts.l map $.sku, right: parts.r map $.sku}
    },
    grouped: byDept pluck $$,
    deptTotals: deptTotals,
    deptEvery: deptTotals everyEntry ($ > 0),
    deptSome: deptTotals someEntry ($ > 10),
    merged: mergeWith({tools: 9}, {kitchen: 15}),
    namesByDept: byDept mapObject { ($$): $ map $.sku },
    leafNames: tree mapLeafValues (value, path) ->
      if (value is String) upper(value) else value,
    nonEmptyTree: tree filterTree ((value, path) ->
      if (value is String) !isEmpty(value) else true),
    hasNorth: tree nodeExists ((value, path) ->
      path[-1].selector == "name" and value == "north"),
    hasWest: tree nodeExists ((value, path) ->
      path[-1].selector == "name" and value == "west"),
    pathIsArray: isArrayType([
      {kind: OBJECT_TYPE, selector: "warehouse", namespace: null},
      {kind: ARRAY_TYPE, selector: 0, namespace: null}
    ]),
    expression: asExpressionString([
      {kind: OBJECT_TYPE, selector: "warehouse", namespace: null},
      {kind: OBJECT_TYPE, selector: "zones", namespace: null}
    ])
  }
}
```

Run it from the repository root:

```bash
./target/debug/rustweave -f examples/scripts/combined.dwl
```

Expected output:

```json
{
  "openSkus": [
    "r1",
    "r3",
    "r4"
  ],
  "totalQty": 24,
  "openCount": 3,
  "allPositive": true,
  "anyLow": true,
  "firstLow": "r2",
  "split": {
    "left": [
      "r1",
      "r2"
    ],
    "right": [
      "r3",
      "r4"
    ]
  },
  "grouped": [
    "tools",
    "kitchen"
  ],
  "deptTotals": {
    "tools": 9,
    "kitchen": 15
  },
  "deptEvery": true,
  "deptSome": true,
  "merged": {
    "tools": 9,
    "kitchen": 15
  },
  "namesByDept": {
    "tools": [
      "r1",
      "r2"
    ],
    "kitchen": [
      "r3",
      "r4"
    ]
  },
  "leafNames": {
    "warehouse": {
      "zones": [
        {
          "name": "NORTH",
          "stock": [
            {
              "sku": "R1",
              "dept": "TOOLS",
              "qty": 7,
              "open": true
            },
            {
              "sku": "R2",
              "dept": "TOOLS",
              "qty": 2,
              "open": false
            },
            {
              "sku": "R3",
              "dept": "KITCHEN",
              "qty": 11,
              "open": true
            },
            {
              "sku": "R4",
              "dept": "KITCHEN",
              "qty": 4,
              "open": true
            }
          ]
        },
        {
          "name": "SOUTH",
          "stock": [
            
          ]
        }
      ]
    }
  },
  "nonEmptyTree": {
    "warehouse": {
      "zones": [
        {
          "name": "north",
          "stock": [
            {
              "sku": "r1",
              "dept": "tools",
              "qty": 7,
              "open": true
            },
            {
              "sku": "r2",
              "dept": "tools",
              "qty": 2,
              "open": false
            },
            {
              "sku": "r3",
              "dept": "kitchen",
              "qty": 11,
              "open": true
            },
            {
              "sku": "r4",
              "dept": "kitchen",
              "qty": 4,
              "open": true
            }
          ]
        },
        {
          "name": "south",
          "stock": [
            
          ]
        }
      ]
    }
  },
  "hasNorth": true,
  "hasWest": false,
  "pathIsArray": true,
  "expression": ".warehouse.zones"
}
```
