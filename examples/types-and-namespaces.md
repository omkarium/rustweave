# Declared types, namespace prefixes, and output forms

This guide shows type aliases used with `as` and `is`, a namespace prefix that is dropped for JSON
output, and the `json` output format ID.

## Script

```dataweave
%dw 2.0
ns cat http://example.test/catalog
output json
type Sku = String
type Item = {sku: Sku, qty: Number, note?: String}
type Batch = Array<Item>
---
{
  assigned: {sku: 'A-1', qty: 2} as Item,
  extra: {sku: 'A-2', qty: 5, region: 'north'} as Item,
  valid: {sku: 'A-3', qty: 1} is Item,
  badShape: {sku: 9, qty: 1} is Item,
  batch: [{sku: 'A-4', qty: 3}] as Batch,
  codes: {cat#label: 'parts', cat#count: 2}
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/types-and-namespaces.dwl
```

## Expected output

```json
{
  "assigned": {
    "sku": "A-1",
    "qty": 2
  },
  "extra": {
    "sku": "A-2",
    "qty": 5,
    "region": "north"
  },
  "valid": true,
  "badShape": false,
  "batch": [
    {
      "sku": "A-4",
      "qty": 3
    }
  ],
  "codes": {
    "label": "parts",
    "count": 2
  }
}
```

## Related forms

The same script can use the `xml` format ID to keep a namespace prefix in the output:

```dataweave
%dw 2.0
ns cat http://example.test/catalog
output xml writeDeclaration=false
---
{cat#label: 'parts'}
```

```xml
<cat:label xmlns:cat="http://example.test/catalog">parts</cat:label>
```

The `with` clause selects a writer while the declared type before it is unchanged:

```dataweave
%dw 2.0
output application/problem+json with csv
---
[{sku: 'A-1', qty: 2}]
```

```text
sku,qty
A-1,2
```

Supported format IDs are `json`, `csv`, `xml`, `dw`, and `text`. Supported writers are the same
names. Writer properties such as `indent=false` still apply after `with`.

## Features shown

`type` aliases with built-in, object, array, union, and intersection types; `as` coercion and
structural `is`; value metadata after an alias; `ns` declarations in the header and `do` blocks;
`prefix#name` keys; JSON prefix removal; XML namespace declarations; output format IDs; and
`output ... with writer`.
