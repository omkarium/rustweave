%dw 2.0
import * from dw::extra::Common
output application/json
---
do {
  var ordered = sorted(payload.queue, (ticket) -> ticket.priority)
  var minutes = ordered map $.minutes
  var highPriority = binarySearch(ordered map $.priority, 4)
  ---
  {
    order: ordered map (ticket) -> {ticket: ticket.ticket, priority: ticket.priority},
    highPriorityStart: highPriority,
    rollingMinutes: windows(minutes, 3)
      map (group) -> toFixed(fold(group, 0, (n, sum) -> sum + n) / sizeOf(group), 1),
    sampled: stepBy(ordered, 2) map $.ticket,
    fullBatches: chunksExact(ordered, 3) map (batch) -> batch map $.ticket,
    firstBatch: truncate(ordered, 3) map $.ticket,
    rotatedLeft: rotateLeft(ordered, 1) map $.ticket,
    rotatedRight: rotateRight(ordered, 2) map $.ticket,
    swapped: swap(minutes, 0, 2),
    filled: fill(minutes, 0, 3, 5),
    nextHigh: position(ordered, (ticket) -> ticket.priority >= 4),
    lastLow: rposition(ordered, (ticket) -> ticket.priority <= 2),
    consecutiveZones: dedup(ordered map $.zone),
    zoneMinutes: fromEntries(
      unique(ordered map $.zone) map (zone) -> {
        key: zone,
        value: fold(ordered filter ($.zone == zone) map $.minutes, 0, (n, sum) -> sum + n)
      }
    )
  }
}
