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
