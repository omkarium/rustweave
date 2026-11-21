# Error handling with `dw::Runtime`

This guide shows `failIf`, `try`, `orElse`, and `orElseTry` in one script. User errors and
runtime errors are captured as `TryResult` values instead of stopping evaluation.

## Script

```dataweave
%dw 2.0
import * from dw::Runtime
output application/json
---
{
  guarded: failIf(3, (n) -> n > 5),
  ok: try(() -> 8 / 2),
  user: try(() -> fail("declined")).error.kind,
  arithmetic: try(() -> 5 / 0).error.kind,
  primary: orElse(try(() -> "ready"), () -> "fallback"),
  fallback: orElse(try(() -> fail("missing")), () -> "fallback"),
  fallbackTry: orElseTry(try(() -> fail("missing")), () -> 7),
  failedTry: orElseTry(try(() -> fail("first")), () -> fail("second")).error.message
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/runtime-errors.dwl
```

## Expected output

```json
{
  "guarded": 3,
  "ok": {
    "success": true,
    "result": 4
  },
  "user": "UserException",
  "arithmetic": "DivisionByZeroException",
  "primary": "ready",
  "fallback": "fallback",
  "fallbackTry": {
    "success": true,
    "result": 7
  },
  "failedTry": "second"
}
```

## Result shape

`try` returns `{success: true, result: value}` on success or
`{success: false, error: {kind, message}}` on failure. A successful result has a `null` `error`
field, and a failed result has a `null` `result` field when selected. `orElse` unwraps a successful
result or evaluates its fallback, while `orElseTry` always returns a `TryResult`.

The error object may also carry runtime-specific location and stack information. Those fields are
not part of the supported contract.

## Features shown

`fail`, `failIf`, `try`, `orElse`, and `orElseTry` from `dw::Runtime`.
