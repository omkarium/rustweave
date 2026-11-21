%dw 2.0
import * from dw::core::Arrays
output application/json
---
do {
  var orders = [
    {sku: "r1", qty: 7, active: true},
    {sku: "r2", qty: 2, active: false},
    {sku: "r3", qty: 11, active: true},
    {sku: "r4", qty: 4, active: true}
  ]
  var people = [
    {key: "p1", label: "Nora"},
    {key: "p2", label: "Silas"},
    {key: "p3", label: "Ivy"}
  ]
  var tasks = [
    {owner: "p1", title: "Tide chart"},
    {owner: "p1", title: "Soil grid"},
    {owner: "p3", title: "Wind map"},
    {owner: "p9", title: "Unassigned"}
  ]
  ---
  {
    countBy: orders countBy $.active,
    sumBy: orders sumBy $.qty,
    some: orders some ($.qty > 10),
    every: orders every ($.qty > 0),
    take: orders take 2 map $.sku,
    drop: orders drop 2 map $.sku,
    slice: slice([10, 20, 30, 40, 50], 1, 4),
    indexOf: indexOf(["r1", "r2", "r3"], "r2"),
    indexWhere: orders indexWhere ($.qty >= 10),
    splitAt: do {
      var parts = orders splitAt 2
      ---
      {left: parts.l map $.sku, right: parts.r map $.sku}
    },
    takeWhile: orders takeWhile ($.qty < 10) map $.sku,
    dropWhile: orders dropWhile ($.qty < 10) map $.sku,
    partition: orders partition ($.active),
    firstWith: (orders firstWith ($.qty > 10)).sku,
    divideBy: divideBy([1, 2, 3, 4, 5, 6, 7], 3),
    splitWhere: splitWhere([1, 2, 3, 4, 5], (x) -> x > 3),
    join: join(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> pair.l.label ++ ": " ++ pair.r.title,
    leftJoin: leftJoin(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> pair.l.label,
    outerJoin: outerJoin(people, tasks, (person) -> person.key, (task) -> task.owner)
      map (pair) -> if (pair.l == null) "unassigned" else pair.l.label
  }
}
