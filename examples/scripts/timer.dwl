%dw 2.0
import * from dw::util::Timer
output application/json
---
{
  fixed: toMilliseconds(|2020-01-01T00:00:00Z|),
  durationKeys: keysOf(duration(() -> 2 + 3)),
  durationResult: duration(() -> 2 + 3).result,
  timeKeys: keysOf(time(() -> "ok")),
  timeResult: time(() -> "ok").result,
  clockPositive: currentMilliseconds() > 0
}
