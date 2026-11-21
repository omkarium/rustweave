# Types functions in one script

Several declared types cover named aliases, literals, arrays, unions, intersections, functions, and objects. The result uses every type-inspection helper in the module.

This script uses all 37 functions listed for `dw::core::Types`.

Import `dw::core::Types` to use the module functions. The result object uses Core `map` to shorten the field list.

## Script

```dataweave
%dw 2.0
import * from dw::core::Types
type Invoice = String {format: "uuuu-MM-dd"}
type Status = "paid"
type Codes = Array<String>
type NestedCodes = Array<Codes>
type Mixed = String | Number
type Both = {x: String} & {y: Number}
type Calc = (String, Number) -> Number
type EmptyCalc = () -> Number
type Shape = {name: String, age?: Number, tags*: String}
output application/json
---
{
  base: baseTypeOf(Invoice),
  metadata: metadataOf(Invoice),
  literal: literalValueOf(Status),
  name: nameOf(Invoice),
  builtInName: nameOf(String),
  item: arrayItem(Codes),
  anyItem: arrayItem(Array),
  union: unionItems(Mixed),
  intersection: intersectionItems(Both),
  params: functionParamTypes(Calc),
  noParams: functionParamTypes(EmptyCalc),
  return: functionReturnType(Calc),
  fields: objectFields(Shape) map $.key.name.localName,
  reference: [
    isReferenceType(Codes),
    isReferenceType(arrayItem(NestedCodes)),
    isReferenceType(String)
  ],
  predicates: {
    any: isAnyType(Any),
    array: isArrayType(Codes),
    binary: isBinaryType(Binary),
    boolean: isBooleanType(Boolean),
    dateTime: isDateTimeType(DateTime),
    date: isDateType(Date),
    function: isFunctionType(Calc),
    intersection: isIntersectionType(Both),
    key: isKeyType(Key),
    literal: isLiteralType(Status),
    localDateTime: isLocalDateTimeType(LocalDateTime),
    localTime: isLocalTimeType(LocalTime),
    namespace: isNamespaceType(Namespace),
    nothing: isNothingType(Nothing),
    null: isNullType(Null),
    number: isNumberType(Number),
    object: isObjectType(Shape),
    period: isPeriodType(Period),
    range: isRangeType(Range),
    regex: isRegexType(Regex),
    string: isStringType(String),
    time: isTimeType(Time),
    timeZone: isTimeZoneType(TimeZone),
    type: isTypeType(Type),
    union: isUnionType(Mixed),
    uri: isUriType(Uri)
  }
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/types.dwl
```

## Expected output

```json
{
  "base": "String",
  "metadata": {
    "format": "uuuu-MM-dd"
  },
  "literal": "paid",
  "name": "Invoice",
  "builtInName": "String",
  "item": "String",
  "anyItem": "Any",
  "union": [
    "String",
    "Number"
  ],
  "intersection": [
    "Object",
    "Object"
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
  "noParams": [
    
  ],
  "return": "Number",
  "fields": [
    "name",
    "age",
    "tags"
  ],
  "reference": [
    false,
    true,
    false
  ],
  "predicates": {
    "any": true,
    "array": true,
    "binary": true,
    "boolean": true,
    "dateTime": true,
    "date": true,
    "function": true,
    "intersection": true,
    "key": true,
    "literal": true,
    "localDateTime": true,
    "localTime": true,
    "namespace": true,
    "nothing": true,
    "null": true,
    "number": true,
    "object": true,
    "period": true,
    "range": true,
    "regex": true,
    "string": true,
    "time": true,
    "timeZone": true,
    "type": true,
    "union": true,
    "uri": true
  }
}
```

## Functions shown

`arrayItem`, `baseTypeOf`, `functionParamTypes`, `functionReturnType`, `intersectionItems`, `isAnyType`, `isArrayType`, `isBinaryType`, `isBooleanType`, `isDateTimeType`, `isDateType`, `isFunctionType`, `isIntersectionType`, `isKeyType`, `isLiteralType`, `isLocalDateTimeType`, `isLocalTimeType`, `isNamespaceType`, `isNothingType`, `isNullType`, `isNumberType`, `isObjectType`, `isPeriodType`, `isRangeType`, `isReferenceType`, `isRegexType`, `isStringType`, `isTimeType`, `isTimeZoneType`, `isTypeType`, `isUnionType`, `isUriType`, `literalValueOf`, `metadataOf`, `nameOf`, `objectFields`, `unionItems`
