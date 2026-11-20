// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{run, run_result};

#[test]
fn parse_numbers() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  decimal: parseInt("  -42px"),
  hex: parseInt("+2fZ", 16),
  binary: parseInt("101012", 2),
  prefixed: parseInt("0x10"),
  empty: parseInt("   "),
  invalid: parseInt("nope"),
  float: parseFloat("  -12.75e2rest"),
  fraction: parseFloat("+.5"),
  exponent: parseFloat("3.5E+2"),
  noExponentDigits: parseFloat("1e"),
  infinity: parseFloat("Infinity"),
  notANumber: parseFloat("NaN"),
  hexFloat: parseFloat("0x10")
}"####;
    let expected = r####"{
  "decimal": -42,
  "hex": 47,
  "binary": 21,
  "prefixed": 0,
  "empty": null,
  "invalid": null,
  "float": -1275,
  "fraction": 0.5,
  "exponent": 350,
  "noExponentDigits": 1,
  "infinity": null,
  "notANumber": null,
  "hexFloat": 0
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn fixed_clamp_trunc_sign() {
    let script = r####"%dw 2.0
import * from dw::extra::Common
output application/json
---
{
  rounded: toFixed(2.345, 2),
  negative: toFixed(-1.5),
  padded: toFixed(7, 2),
  halfUp: toFixed(0.005, 2),
  negativeZero: toFixed(-0.004, 2),
  defaultDigits: toFixed(12),
  inside: clamp(7, 3, 9),
  below: clamp(-5, -2, 4),
  above: clamp(12, -2, 4),
  truncPositive: trunc(3.9),
  truncNegative: trunc(-3.9),
  truncFraction: trunc(-0.5),
  signNegative: sign(-5),
  signZero: sign(0),
  signPositive: sign(2)
}"####;
    let expected = r####"{
  "rounded": "2.35",
  "negative": "-2",
  "padded": "7.00",
  "halfUp": "0.01",
  "negativeZero": "0.00",
  "defaultDigits": "12",
  "inside": 7,
  "below": -2,
  "above": 4,
  "truncPositive": 3,
  "truncNegative": -3,
  "truncFraction": 0,
  "signNegative": -1,
  "signZero": 0,
  "signPositive": 1
}"####;
    assert_eq!(run(script), expected);
}

#[test]
fn number_argument_errors() {
    for script in [
        r####"parseInt(1)"####,
        r####"parseInt("1", 2.5)"####,
        r####"parseInt("1", 1)"####,
        r####"parseInt("1", 37)"####,
        r####"parseFloat(1)"####,
        r####"toFixed("1", 2)"####,
        r####"toFixed(1, -1)"####,
        r####"toFixed(1, 21)"####,
        r####"toFixed(1, 1.5)"####,
        r####"clamp(1, 5, 2)"####,
        r####"clamp("1", 0, 2)"####,
        r####"trunc(null)"####,
        r####"sign(null)"####,
    ] {
        let script = format!("%dw 2.0\nimport * from dw::extra::Common\n---\n{script}");
        assert!(run_result(&script).is_err(), "expected error: {script}");
    }
}
