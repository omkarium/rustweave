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
