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
