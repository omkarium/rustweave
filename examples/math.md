# Math functions in one script

Trigonometric, logarithmic, decimal, and angle conversions are evaluated in one result object. Invalid trigonometric and logarithm domains are covered by the module tests, not this sample.

This script uses all 17 functions listed for `dw::util::Math`.

Import `dw::util::Math` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::util::Math
output application/json
---
{
  acos: acos(0.5),
  asin: asin(0.5),
  atan: atan(0.5),
  cos: cos(0.5),
  sin: sin(0.5),
  tan: tan(0.5),
  log10: log10(1000),
  logn: logn(10),
  decimal: {
    add: decimalAdd(2.3, 4.5),
    subtract: decimalSubtract(9, 2.75),
    multiply: decimalMultiply(2.5, 4),
    divide: decimalDivide(1, 8),
    pow: decimalPow(3, 4),
    sqrt: decimalSqrt(16),
    roundUp: decimalRound(4.5, {precision: 1, roundingMode: "HALF_UP"}),
    roundEven: decimalRound(4.5, {precision: 1, roundingMode: "HALF_EVEN"})
  },
  degrees: toDegrees(3.141592653589793),
  radians: toRadians(180)
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/math.dwl
```

## Expected output

```json
{
  "acos": 1.0471975511965979,
  "asin": 0.5235987755982989,
  "atan": 0.4636476090008061,
  "cos": 0.8775825618903728,
  "sin": 0.479425538604203,
  "tan": 0.5463024898437905,
  "log10": 3,
  "logn": 2.302585092994046,
  "decimal": {
    "add": 6.8,
    "subtract": 6.25,
    "multiply": 10,
    "divide": 0.125,
    "pow": 81,
    "sqrt": 4,
    "roundUp": 5,
    "roundEven": 4
  },
  "degrees": 180,
  "radians": 3.141592653589793
}
```

## Functions shown

`acos`, `asin`, `atan`, `cos`, `decimalAdd`, `decimalDivide`, `decimalMultiply`, `decimalPow`, `decimalRound`, `decimalSqrt`, `decimalSubtract`, `log10`, `logn`, `sin`, `tan`, `toDegrees`, `toRadians`
