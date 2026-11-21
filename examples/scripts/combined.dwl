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
