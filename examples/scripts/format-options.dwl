%dw 2.0
output application/json
---
{
  noHeader: read("10,20\n30,40", "application/csv", {header: false}),
  headerLine: read("skip\ncode,qty\nK-1,4", "application/csv", {headerLineNumber: 2}),
  escaped: write([{note: "left,right"}], "application/csv", {escape: "\\"}),
  unquoted: write([{note: "left,right"}], "application/csv", {quoteValues: false}),
  emptyRow: write([{a: 1}, {}, {a: 3}], "application/csv", {header: false}),
  duplicates: write({a: 1, a: 2}, "application/json", {duplicateKeyAsArray: true}),
  defaultNamespace: write({part: 7}, "application/xml", {writeDeclaration: false, defaultNamespace: "http://sample.test/ns"}),
  nil: write({part: null}, "application/xml", {writeDeclaration: false, writeNilOnNull: true}),
  cr: write({note: "a\rb"}, "application/xml", {writeDeclaration: false, escapeCR: true}),
  skipNull: write({empty: null, part: 7}, "application/xml", {writeDeclaration: false, skipNullOn: "elements"})
}
