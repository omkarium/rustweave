# Reader and writer options for JSON, CSV, and XML

This guide shows the option fixes for CSV headers, CSV escaping, JSON duplicate keys, and XML
namespaces, null elements, and carriage returns. It calls `read` and `write` with explicit
properties so every option fits in one script.

## Script

```dataweave
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
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/format-options.dwl
```

## Expected output

```json
{
  "noHeader": [
    {
      "column_0": "10",
      "column_1": "20"
    },
    {
      "column_0": "30",
      "column_1": "40"
    }
  ],
  "headerLine": [
    {
      "code": "K-1",
      "qty": "4"
    }
  ],
  "escaped": "note\nleft\\,right\n",
  "unquoted": "note\nleft\\,right\n",
  "emptyRow": "1\n\n3\n",
  "duplicates": "{\n  \"a\": [\n  1,\n  2\n  ]\n}",
  "defaultNamespace": "<part xmlns=\"http://sample.test/ns\">7</part>",
  "nil": "<part xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:nil=\"true\"/>",
  "cr": "<note>a&#xd;b</note>",
  "skipNull": "<part>7</part>"
}
```

## Notes

- CSV `header=false` names columns `column_0`, `column_1`, and so on. `headerLineNumber` selects the
  header line and skips earlier lines.
- CSV `escape` and `quoteValues=false` escape separators with the escape character. An empty object
  row emits an empty line.
- JSON `duplicateKeyAsArray=true` groups repeated keys into arrays with the reference indentation.
- XML `defaultNamespace` adds a default `xmlns` on the root, `writeNilOnNull` writes `xsi:nil`, and
  `escapeCR` writes `&#xd;`. `skipNullOn="elements"` omits null elements.

The same options can be used in the `output` header, for example
`output application/csv escape="\\"` or
`output application/xml writeDeclaration=false, defaultNamespace="http://sample.test/ns"`.

## Features shown

CSV reader `header` and `headerLineNumber`; CSV writer `escape`, `quoteValues`, and empty object
rows; JSON writer `duplicateKeyAsArray`; XML writer `defaultNamespace`, `writeNilOnNull`, `escapeCR`,
and `skipNullOn`.
