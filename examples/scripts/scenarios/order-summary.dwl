%dw 2.0
import * from dw::extra::Common
output application/json
---
do {
  var paid = payload.orders filter ($.status == "paid")
  var priced = paid map (order) -> assign(order, {
    total: order.units * order.unitPrice,
    label: zfill(order.id, 7)
  })
  var ranked = sorted(priced, (order) -> order.total, true)
  var regions = unique(ranked map $.region)
  ---
  {
    regions: regions map (region) -> do {
      var lines = ranked filter ($.region == region)
      ---
      {
        name: title(region),
        orders: lines map (order) -> pick(order, ["label", "units", "total"]),
        units: fold(lines, 0, (order, sum) -> sum + order.units),
        revenue: toFixed(fold(lines, 0, (order, sum) -> sum + order.total), 2),
        notes: unique(compact(lines map $.note))
      }
    },
    topOrder: {
      label: ranked[0].label,
      total: toFixed(ranked[0].total, 2)
    },
    lead: toFixed(ranked[0].total - ranked[-1].total, 2),
    labels: ranked map $.label,
    paused: payload.orders
      filter ($.status != "paid")
      map (order) -> omit(order, ["note", "region"])
  }
}
