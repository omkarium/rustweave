// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for `dw::util::Tree`.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: isArrayType([{kind: OBJECT_TYPE, selector: "invoice", namespace: null}, {kind: ATTRIBUTE_TYPE, selector: "amount", namespace: null}]),
  b: isArrayType([{kind: OBJECT_TYPE, selector: "invoice", namespace: null}, {kind: ARRAY_TYPE, selector: 0, namespace: null}]),
  c: isAttributeType([{kind: ATTRIBUTE_TYPE, selector: "amount", namespace: null}]),
  d: isObjectType([{kind: OBJECT_TYPE, selector: "invoice", namespace: null}, {kind: OBJECT_TYPE, selector: "amount", namespace: null}]),
  e: asExpressionString([{kind: OBJECT_TYPE, selector: "invoice", namespace: null}, {kind: ATTRIBUTE_TYPE, selector: "amount", namespace: null}])
}"####,
        r####"{
  "a": false,
  "b": true,
  "c": true,
  "d": true,
  "e": ".invoice.@amount"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: {shelf: [{item: "Tea", code: "T-1"}], category: "pantry"} mapLeafValues (value, path) -> upper(value),
  b: {title: "Harbor", tags: [4, 5, 6]} mapLeafValues ((value, path) -> if (isObjectType(path)) "***" else if (isArrayType(path)) "In an array" else "Is an attribute")
}"####,
        r####"{
  "a": {
    "shelf": [
      {
        "item": "TEA",
        "code": "T-1"
      }
    ],
    "category": "PANTRY"
  },
  "b": {
    "title": "***",
    "tags": [
      "In an array",
      "In an array",
      "In an array"
    ]
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Tree
var rows = [3, {name: ["", false], count: 17}, "42", null]
var nodes = [{name: "", id: "x"}, {name: "Ok", id: null}]
output application/json
---
{
  a: rows filterArrayLeafs ((value, path) -> !(value is Null or value is String)),
  b: rows filterArrayLeafs ((value, path) -> (value is Null or value == 3)),
  c: {k: [3, 4]} filterArrayLeafs ((value, path) -> (value is Null or value == 3)),
  d: rows filterArrayLeafs ((value, path) -> !isArrayType(path)),
  e: {name: "Rook", lastName: null, age: 45, links: nodes} filterObjectLeafs ((value, path) -> !(value is Null or value is String)),
  f: {left: null, right: "keep"} filterObjectLeafs ((value, path) -> (value is Null and isObjectType(path)))
}"####,
        r####"{
  "a": [
    3,
    {
      "name": [
        false
      ],
      "count": 17
    }
  ],
  "b": [
    3,
    {
      "name": [
        
      ],
      "count": 17
    },
    null
  ],
  "c": {
    "k": [
      3
    ]
  },
  "d": [
    {
      "name": [
        
      ],
      "count": 17
    }
  ],
  "e": {
    "age": 45,
    "links": [
      {
        
      },
      {
        
      }
    ]
  },
  "f": {
    "left": null
  }
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: {name: "", lastName: "Rook", links: [{id: "", active: true}, {age: 45}, ""]} filterTree ((value, path) -> if (value is String) !isEmpty(value) else true),
  b: null filterTree ((value, path) -> value is String),
  c: [{name: "Rook", links: []}, {test: [4, 5, 6]}, {code: ""}] filterTree ((value, path) -> if (value is Array) !isEmpty(value as Array) else true)
}"####,
        r####"{
  "a": {
    "lastName": "Rook",
    "links": [
      {
        "active": true
      },
      {
        "age": 45
      }
    ]
  },
  "b": null,
  "c": [
    {
      "name": "Rook"
    },
    {
      "test": [
        4,
        5,
        6
      ]
    },
    {
      "code": ""
    }
  ]
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::util::Tree
var catalog = {shelf: [{name: "Tea", code: "T-1", tags: [{name: "green"}, {name: "loose"}]}, {name: "Rope", code: "R-2", tags: [{name: "nylon"}, {name: "braid"}]}]}
output application/json
---
{
  tea: catalog nodeExists ((value, path) -> path[-1].selector == "name" and value == "Tea"),
  green: catalog nodeExists ((value, path) -> path[-1].selector == "name" and value == "green"),
  loose: catalog nodeExists ($$[-1].selector == "name" and $ == "loose"),
  braid: catalog nodeExists ($$[-1].selector == "name" and $ == "braid"),
  wrongField: catalog nodeExists ($$[-1].selector == "brand"),
  coffee: catalog nodeExists ($$[-1].selector == "name" and $ == "Coffee")
}"####,
        r####"{
  "tea": true,
  "green": true,
  "loose": true,
  "braid": true,
  "wrongField": false,
  "coffee": false
}"####,
    ),
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
}
