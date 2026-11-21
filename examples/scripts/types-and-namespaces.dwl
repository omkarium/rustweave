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
