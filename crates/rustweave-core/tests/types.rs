// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for `dw::core::Types`.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn literal_metadata_name_and_base() {
    let script = r#"%dw 2.0
import * from dw::core::Types
type AType = String {format: "YYYY-MM-dd"}
type ALiteral = "Mariano"
type ANumber = Number
type AArray = Array<String> {n: 1}
output application/json
---
{
  base: baseTypeOf(AType),
  metadata: metadataOf(AType),
  literal: literalValueOf(ALiteral),
  literalType: isLiteralType(ALiteral),
  name: nameOf(AType),
  arrayName: nameOf(AArray),
  numberName: nameOf(ANumber),
  stringName: nameOf(String)
}"#;
    let expected = r#"{
  "base": "String",
  "metadata": {
    "format": "YYYY-MM-dd"
  },
  "literal": "Mariano",
  "literalType": true,
  "name": "AType",
  "arrayName": "AArray",
  "numberName": "ANumber",
  "stringName": "String"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn scalar_type_predicates() {
    let script = r#"%dw 2.0
import * from dw::core::Types
type AAny = Any
type ABinary = Binary
type ABoolean = Boolean
type ADate = Date
type ADateTime = DateTime
type AKey = Key
type ALocalDateTime = LocalDateTime
type ALocalTime = LocalTime
type ANamespace = Namespace
type ANothing = Nothing
type ANull = Null
type APeriod = Period
type ARange = Range
type ARegex = Regex
type ATime = Time
type ATimeZone = TimeZone
type AUri = Uri
type AType = Type
output application/json
---
{
  any: [isAnyType(AAny), isAnyType(Any), isAnyType(String)],
  binary: [isBinaryType(ABinary), isBinaryType(Binary), isBinaryType(String)],
  boolean: [isBooleanType(ABoolean), isBooleanType(Boolean), isBooleanType(String)],
  date: [isDateType(ADate), isDateType(Date), isDateType(String)],
  dateTime: [isDateTimeType(ADateTime), isDateTimeType(DateTime), isDateTimeType(String)],
  key: [isKeyType(AKey), isKeyType(Key), isKeyType(Boolean)],
  localDateTime: [isLocalDateTimeType(ALocalDateTime), isLocalDateTimeType(LocalDateTime), isLocalDateTimeType(String)],
  localTime: [isLocalTimeType(ALocalTime), isLocalTimeType(LocalTime), isLocalTimeType(String)],
  namespace: [isNamespaceType(ANamespace), isNamespaceType(Namespace), isNamespaceType(String)],
  nothing: [isNothingType(ANothing), isNothingType(Nothing), isNothingType(String)],
  null: [isNullType(ANull), isNullType(Null), isNullType(String)],
  number: [isNumberType(Number), isNumberType(Integer), isNumberType(String)],
  period: [isPeriodType(APeriod), isPeriodType(Period), isPeriodType(String)],
  range: [isRangeType(ARange), isRangeType(Range), isRangeType(String)],
  regex: [isRegexType(ARegex), isRegexType(Regex), isRegexType(Boolean)],
  time: [isTimeType(ATime), isTimeType(Time), isTimeType(String)],
  timeZone: [isTimeZoneType(ATimeZone), isTimeZoneType(TimeZone), isTimeZoneType(String)],
  type: [isTypeType(AType), isTypeType(Type), isTypeType(String)],
  uri: [isUriType(AUri), isUriType(Uri), isUriType(String)]
}"#;
    let expected = r#"{
  "any": [
    true,
    true,
    false
  ],
  "binary": [
    true,
    true,
    false
  ],
  "boolean": [
    true,
    true,
    false
  ],
  "date": [
    true,
    true,
    false
  ],
  "dateTime": [
    true,
    true,
    false
  ],
  "key": [
    true,
    true,
    false
  ],
  "localDateTime": [
    true,
    true,
    false
  ],
  "localTime": [
    true,
    true,
    false
  ],
  "namespace": [
    true,
    true,
    false
  ],
  "nothing": [
    true,
    true,
    false
  ],
  "null": [
    true,
    true,
    false
  ],
  "number": [
    true,
    true,
    false
  ],
  "period": [
    true,
    true,
    false
  ],
  "range": [
    true,
    true,
    false
  ],
  "regex": [
    true,
    true,
    false
  ],
  "time": [
    true,
    true,
    false
  ],
  "timeZone": [
    true,
    true,
    false
  ],
  "type": [
    true,
    true,
    false
  ],
  "uri": [
    true,
    true,
    false
  ]
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn composite_type_helpers() {
    let script = r#"%dw 2.0
import * from dw::core::Types
type AArray = Array<String>
type AArray2 = Array<AArray>
type AUnion = String | Number
type AIntersection = {name: String} & {age: Number}
type AFunction = (String, Number) -> Number
type AFunction2 = () -> Number
type AObject = {name: String, age?: Number, tags*: String}
output application/json
---
{
  array: [isArrayType(AArray), isArrayType(Boolean)],
  object: [isObjectType(AObject), isObjectType(Boolean)],
  arrayItem: [arrayItem(AArray), arrayItem(Array)],
  union: unionItems(AUnion),
  unionType: [isUnionType(AUnion), isUnionType(Boolean)],
  intersection: intersectionItems(AIntersection),
  intersectionType: [isIntersectionType(AIntersection), isIntersectionType(Boolean)],
  functionType: [isFunctionType(AFunction), isFunctionType(Boolean)],
  params: functionParamTypes(AFunction),
  emptyParams: functionParamTypes(AFunction2),
  return: [functionReturnType(AFunction), functionReturnType(AFunction2)],
  reference: [isReferenceType(AArray), isReferenceType(arrayItem(AArray2)), isReferenceType(String)],
  fields: objectFields(AObject),
  emptyFields: objectFields(Object)
}"#;
    let expected = r#"{
  "array": [
    true,
    false
  ],
  "object": [
    true,
    false
  ],
  "arrayItem": [
    "String",
    "Any"
  ],
  "union": [
    "String",
    "Number"
  ],
  "unionType": [
    true,
    false
  ],
  "intersection": [
    "Object",
    "Object"
  ],
  "intersectionType": [
    true,
    false
  ],
  "functionType": [
    true,
    false
  ],
  "params": [
    {
      "paramType": "String",
      "optional": false
    },
    {
      "paramType": "Number",
      "optional": false
    }
  ],
  "emptyParams": [
    
  ],
  "return": [
    "Number",
    "Number"
  ],
  "reference": [
    false,
    true,
    false
  ],
  "fields": [
    {
      "key": {
        "name": {
          "localName": "name",
          "namespace": null
        },
        "attributes": [
          
        ]
      },
      "required": true,
      "repeated": false,
      "value": "String"
    },
    {
      "key": {
        "name": {
          "localName": "age",
          "namespace": null
        },
        "attributes": [
          
        ]
      },
      "required": false,
      "repeated": false,
      "value": "Number"
    },
    {
      "key": {
        "name": {
          "localName": "tags",
          "namespace": null
        },
        "attributes": [
          
        ]
      },
      "required": true,
      "repeated": true,
      "value": "String"
    }
  ],
  "emptyFields": [
    
  ]
}"#;
    assert_eq!(run(script), expected);
}
