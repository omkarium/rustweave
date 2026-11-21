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
