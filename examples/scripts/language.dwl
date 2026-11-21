%dw 2.0
output application/json
var tag = "tree"
---
{
  usingBasic: using (x = 3) x + 1,
  usingChain: using (a = 2, b = a * 4) [a, b],
  spread: {id: 1 ~ {name: "oak"}},
  dynamicSpread: { (tag): true ~ {height: 12} },
  metaValue: (5 as Number {unit: "cm"}).^unit,
  metaAll: (1 as Number {a: 1, b: 2}).^,
  metaMissing: (1).^x
}
