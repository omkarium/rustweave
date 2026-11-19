// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for `dw::util::Tree`.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn path_predicates_and_expression_string() {
    let script = r#"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: isArrayType([{kind: OBJECT_TYPE, selector: "user", namespace: null}, {kind: ATTRIBUTE_TYPE, selector: "name", namespace: null}]),
  b: isArrayType([{kind: OBJECT_TYPE, selector: "user", namespace: null}, {kind: ARRAY_TYPE, selector: 0, namespace: null}]),
  c: isAttributeType([{kind: ATTRIBUTE_TYPE, selector: "name", namespace: null}]),
  d: isObjectType([{kind: OBJECT_TYPE, selector: "user", namespace: null}, {kind: OBJECT_TYPE, selector: "name", namespace: null}]),
  e: asExpressionString([{kind: OBJECT_TYPE, selector: "user", namespace: null}, {kind: ATTRIBUTE_TYPE, selector: "name", namespace: null}])
}"#;
    let expected = r#"{
  "a": false,
  "b": true,
  "c": true,
  "d": true,
  "e": ".user.@name"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn map_leaf_values_examples() {
    let script = r#"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: {user: [{name: "mariano", lastName: "achaval"}], group: "data-weave"} mapLeafValues (value, path) -> upper(value),
  b: {name: "Mariano", test: [1,2,3]} mapLeafValues ((value, path) -> if (isObjectType(path)) "***" else if (isArrayType(path)) "In an array" else "Is an attribute")
}"#;
    let expected = r#"{
  "a": {
    "user": [
      {
        "name": "MARIANO",
        "lastName": "ACHAVAL"
      }
    ],
    "group": "DATA-WEAVE"
  },
  "b": {
    "name": "***",
    "test": [
      "In an array",
      "In an array",
      "In an array"
    ]
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn filter_array_and_object_leafs() {
    let script = r#"%dw 2.0
import * from dw::util::Tree
var myArray = [1, {name: ["", true], test: 213}, "123", null]
var myObjects = [{name: "", id: "test"}, {name: "Me", id: null}]
output application/json
---
{
  a: myArray filterArrayLeafs ((value, path) -> !(value is Null or value is String)),
  b: myArray filterArrayLeafs ((value, path) -> (value is Null or value == 1)),
  c: {a: [1,2]} filterArrayLeafs ((value, path) -> (value is Null or value == 1)),
  d: myArray filterArrayLeafs ((value, path) -> !isArrayType(path)),
  e: {name: "Mariano", lastName: null, age: 123, friends: myObjects} filterObjectLeafs ((value, path) -> !(value is Null or value is String)),
  f: {c: null, d: "hello"} filterObjectLeafs ((value, path) -> (value is Null and isObjectType(path)))
}"#;
    let expected = r#"{
  "a": [
    1,
    {
      "name": [
        true
      ],
      "test": 213
    }
  ],
  "b": [
    1,
    {
      "name": [
        
      ],
      "test": 213
    },
    null
  ],
  "c": {
    "a": [
      1
    ]
  },
  "d": [
    {
      "name": [
        
      ],
      "test": 213
    }
  ],
  "e": {
    "age": 123,
    "friends": [
      {
        
      },
      {
        
      }
    ]
  },
  "f": {
    "c": null
  }
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn filter_tree_examples() {
    let script = r#"%dw 2.0
import * from dw::util::Tree
output application/json
---
{
  a: {name: "", lastName: "Achaval", friends: [{id: "", test: true}, {age: 123}, ""]} filterTree ((value, path) -> if (value is String) !isEmpty(value) else true),
  b: null filterTree ((value, path) -> value is String),
  c: [{name: "Mariano", friends: []}, {test: [1,2,3]}, {dw: ""}] filterTree ((value, path) -> if (value is Array) !isEmpty(value as Array) else true)
}"#;
    let expected = r#"{
  "a": {
    "lastName": "Achaval",
    "friends": [
      {
        "test": true
      },
      {
        "age": 123
      }
    ]
  },
  "b": null,
  "c": [
    {
      "name": "Mariano"
    },
    {
      "test": [
        1,
        2,
        3
      ]
    },
    {
      "dw": ""
    }
  ]
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn node_exists_examples() {
    let script = r#"%dw 2.0
import * from dw::util::Tree
var myObject = {user: [{name: "mariano", lastName: "achaval", friends: [{name: "julian"}, {name: "tom"}]}, {name: "leandro", lastName: "shokida", friends: [{name: "peter"}, {name: "robert"}]}]}
output application/json
---
{
  mariano: myObject nodeExists ((value, path) -> path[-1].selector == "name" and value == "mariano"),
  julian: myObject nodeExists ((value, path) -> path[-1].selector == "name" and value == "julian"),
  tom: myObject nodeExists ($$[-1].selector == "name" and $ == "tom"),
  peter: myObject nodeExists ($$[-1].selector == "name" and $ == "peter"),
  wrongField: myObject nodeExists ($$[-1].selector == "wrongField"),
  teo: myObject nodeExists ($$[-1].selector == "name" and $ == "teo")
}"#;
    let expected = r#"{
  "mariano": true,
  "julian": true,
  "tom": true,
  "peter": true,
  "wrongField": false,
  "teo": false
}"#;
    assert_eq!(run(script), expected);
}
