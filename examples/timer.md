# Timer functions in one script

This guide shows every helper exposed by `dw::util::Timer`. Fixed timings come from a literal
`DateTime`; the elapsed and wall-clock helpers are shown through their keys and results.

The script uses all 4 functions listed for `dw::util::Timer`.

## Script

```dataweave
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
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/timer.dwl
```

## Expected output

```json
{
  "fixed": 1577836800000,
  "durationKeys": [
    "time",
    "result"
  ],
  "durationResult": 5,
  "timeKeys": [
    "start",
    "result",
    "end"
  ],
  "timeResult": "ok",
  "clockPositive": true
}
```

## Functions shown

`currentMilliseconds`, `duration`, `time`, `toMilliseconds`
