// SPDX-License-Identifier: MIT OR Apache-2.0
//! Exact decimal numbers with 34-significant-digit division.

use std::cmp::Ordering;
use std::fmt;

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

const DEFAULT_PRECISION: usize = 34;

/// An exact decimal number.
///
/// A value is represented as a sign, a base-10 coefficient, and a power-of-ten
/// exponent. Trailing zeros are normalized away, so equal values have equal
/// representations.
#[derive(Clone)]
pub struct Number {
    negative: bool,
    coeff: BigUint,
    exp: i64,
}

impl Number {
    /// Returns the number zero.
    pub fn zero() -> Self {
        Number {
            negative: false,
            coeff: BigUint::zero(),
            exp: 0,
        }
    }

    /// Creates a number from a signed 64-bit integer.
    pub fn from_i64(value: i64) -> Self {
        if value == 0 {
            return Number::zero();
        }
        Number {
            negative: value < 0,
            coeff: BigUint::from(value.unsigned_abs()),
            exp: 0,
        }
    }

    /// Creates a non-negative number from an unsigned 64-bit integer.
    pub fn from_u64(value: u64) -> Self {
        Number {
            negative: false,
            coeff: BigUint::from(value),
            exp: 0,
        }
    }

    /// Creates a non-negative number from a `usize`.
    pub fn from_usize(value: usize) -> Self {
        Number::from_u64(value as u64)
    }

    /// Converts a finite `f64` to a decimal number.
    ///
    /// Returns `None` for infinities and NaN.
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        Number::parse(&format!("{value}"))
    }

    /// Parses a decimal string with an optional sign, fraction, and exponent.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }
        let (negative, rest) = match text.as_bytes()[0] {
            b'-' => (true, &text[1..]),
            b'+' => (false, &text[1..]),
            _ => (false, text),
        };
        if rest.is_empty() {
            return None;
        }

        let mut mantissa = String::new();
        let mut frac_len: i64 = 0;
        let mut seen_dot = false;
        let mut seen_digit = false;
        let mut chars = rest.chars().peekable();
        while let Some(c) = chars.peek().copied() {
            if c.is_ascii_digit() {
                mantissa.push(c);
                if seen_dot {
                    frac_len += 1;
                }
                seen_digit = true;
                chars.next();
            } else if c == '.' && !seen_dot {
                seen_dot = true;
                chars.next();
            } else {
                break;
            }
        }
        if !seen_digit {
            return None;
        }

        let mut exp_adjust: i64 = 0;
        if let Some(c) = chars.peek().copied() {
            if c == 'e' || c == 'E' {
                chars.next();
                let mut sign = 1i64;
                if let Some(s) = chars.peek().copied() {
                    if s == '+' {
                        chars.next();
                    } else if s == '-' {
                        sign = -1;
                        chars.next();
                    }
                }
                let mut digits = String::new();
                while let Some(d) = chars.peek().copied() {
                    if d.is_ascii_digit() {
                        digits.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if digits.is_empty() {
                    return None;
                }
                exp_adjust = sign * digits.parse::<i64>().ok()?;
            }
        }
        if chars.next().is_some() {
            return None;
        }
        if exp_adjust.unsigned_abs() > 1_000_000 {
            return None;
        }

        let coeff = BigUint::parse_bytes(mantissa.as_bytes(), 10)?;
        let mut number = Number {
            negative,
            coeff,
            exp: exp_adjust - frac_len,
        };
        number.normalize();
        Some(number)
    }

    /// Removes trailing zero digits from the coefficient.
    fn normalize(&mut self) {
        if self.coeff.is_zero() {
            self.negative = false;
            self.exp = 0;
            return;
        }
        let ten = BigUint::from(10u32);
        while (&self.coeff % &ten).is_zero() {
            self.coeff /= &ten;
            self.exp += 1;
        }
    }

    /// Reports whether the number is zero.
    pub fn is_zero(&self) -> bool {
        self.coeff.is_zero()
    }

    /// Reports whether the number is strictly negative.
    pub fn is_negative(&self) -> bool {
        self.negative && !self.is_zero()
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Self {
        let mut result = self.clone();
        result.negative = false;
        result
    }

    /// Returns the negated value.
    pub fn neg(&self) -> Self {
        let mut result = self.clone();
        if !result.is_zero() {
            result.negative = !result.negative;
        }
        result
    }

    /// Adds two numbers exactly.
    pub fn add(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }
        let exp = self.exp.min(other.exp);
        let a = &self.coeff * pow10(self.exp - exp);
        let b = &other.coeff * pow10(other.exp - exp);
        let mut result = if self.negative == other.negative {
            Number {
                negative: self.negative,
                coeff: a + b,
                exp,
            }
        } else if a >= b {
            Number {
                negative: self.negative,
                coeff: a - b,
                exp,
            }
        } else {
            Number {
                negative: other.negative,
                coeff: b - a,
                exp,
            }
        };
        result.normalize();
        result
    }

    /// Subtracts `other` from this number.
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// Multiplies two numbers exactly.
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Number {
            negative: self.negative != other.negative,
            coeff: &self.coeff * &other.coeff,
            exp: self.exp + other.exp,
        };
        result.normalize();
        result
    }

    /// Divides by `other`, rounding to 34 significant digits.
    ///
    /// Returns `None` when `other` is zero.
    pub fn div(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            return None;
        }
        if self.is_zero() {
            return Some(Number::zero());
        }
        let negative = self.negative != other.negative;
        let digits_a = digit_count(&self.coeff);
        let digits_b = digit_count(&other.coeff);
        let scale = DEFAULT_PRECISION as i64 + digits_b - digits_a + 2;
        let (numerator, denominator) = if scale >= 0 {
            (&self.coeff * pow10(scale), other.coeff.clone())
        } else {
            (self.coeff.clone(), &other.coeff * pow10(-scale))
        };
        let quotient = &numerator / &denominator;
        let remainder = &numerator % &denominator;
        let digits = digit_count(&quotient);
        let drop = (digits - DEFAULT_PRECISION as i64).max(0);
        let exp = self.exp - other.exp - scale;
        let (coeff, exp) = if drop > 0 {
            let divisor = pow10(drop);
            let mut coeff = &quotient / &divisor;
            let lost = &quotient % &divisor;
            let scaled = &lost * &denominator + &remainder;
            if (&scaled * BigUint::from(2u32)) >= (&divisor * &denominator) {
                coeff += BigUint::one();
            }
            (coeff, exp + drop)
        } else {
            let mut coeff = quotient;
            if (&remainder * BigUint::from(2u32)) >= denominator {
                coeff += BigUint::one();
            }
            (coeff, exp)
        };
        let mut result = Number {
            negative,
            coeff,
            exp,
        };
        result.normalize();
        Some(result)
    }

    /// Returns the remainder after division, or `None` for a zero divisor.
    pub fn rem(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            return None;
        }
        let exp = self.exp.min(other.exp);
        let a = &self.coeff * pow10(self.exp - exp);
        let b = &other.coeff * pow10(other.exp - exp);
        let mut result = Number {
            negative: self.negative,
            coeff: a % b,
            exp,
        };
        result.normalize();
        Some(result)
    }

    /// Reports whether the number has no fractional part.
    pub fn is_integer(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        if self.exp >= 0 {
            return true;
        }
        let ten = pow10(-self.exp);
        (&self.coeff % ten).is_zero()
    }

    /// Converts to an `i64` when the number is an integer in range.
    pub fn to_i64(&self) -> Option<i64> {
        if !self.is_integer() {
            return None;
        }
        let value = if self.exp >= 0 {
            self.coeff.clone() * pow10(self.exp)
        } else {
            self.coeff.clone() / pow10(-self.exp)
        };
        let result = value.to_i64();
        result.map(|v| if self.negative { -v } else { v })
    }

    /// Converts to the nearest `f64` when representable.
    pub fn to_f64(&self) -> Option<f64> {
        self.to_string().parse().ok()
    }

    /// Returns the largest integer less than or equal to the number.
    pub fn floor(&self) -> Self {
        if self.exp >= 0 || self.is_zero() {
            return self.clone();
        }
        let scale = pow10(-self.exp);
        let q = &self.coeff / &scale;
        let r = &self.coeff % &scale;
        if self.negative && !r.is_zero() {
            let mut result = Number {
                negative: true,
                coeff: q + BigUint::one(),
                exp: 0,
            };
            result.normalize();
            result
        } else {
            let mut result = Number {
                negative: self.negative,
                coeff: q,
                exp: 0,
            };
            result.normalize();
            result
        }
    }

    /// Returns the smallest integer greater than or equal to the number.
    pub fn ceil(&self) -> Self {
        self.neg().floor().neg()
    }

    /// Rounds to the nearest integer, with halves rounded away from zero.
    pub fn round(&self) -> Self {
        let half = Number::parse("0.5").expect("half");
        if self.negative {
            self.neg().add(&half).floor().neg()
        } else {
            self.add(&half).floor()
        }
    }

    /// Returns the square root when the number is non-negative.
    pub fn sqrt(&self) -> Option<Self> {
        if self.negative {
            return None;
        }
        let value = self.to_f64()?;
        Number::from_f64(value.sqrt())
    }

    /// Raises the number to `exponent`, returning `None` when not representable.
    pub fn pow(&self, exponent: &Self) -> Option<Self> {
        if exponent.is_integer() {
            if let Some(e) = exponent.to_i64() {
                if e.unsigned_abs() <= 1_000_000 {
                    if e == 0 {
                        return Some(Number::from_i64(1));
                    }
                    let mut result = Number::from_i64(1);
                    for _ in 0..e.unsigned_abs() {
                        result = result.mul(self);
                    }
                    if e < 0 {
                        return Number::from_i64(1).div(&result);
                    }
                    return Some(result);
                }
            }
        }
        let base = self.to_f64()?;
        let exp = exponent.to_f64()?;
        Number::from_f64(base.powf(exp))
    }

    /// Formats without scientific notation or trailing fractional zeros.
    pub fn to_plain_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let digits = self.coeff.to_str_radix(10);
        let mut out = String::new();
        if self.negative {
            out.push('-');
        }
        if self.exp >= 0 {
            out.push_str(&digits);
            for _ in 0..self.exp {
                out.push('0');
            }
        } else {
            let point = -self.exp;
            let len = digits.len() as i64;
            if len > point {
                let split = (len - point) as usize;
                out.push_str(&digits[..split]);
                out.push('.');
                out.push_str(&digits[split..]);
            } else {
                out.push_str("0.");
                for _ in 0..(point - len) {
                    out.push('0');
                }
                out.push_str(&digits);
            }
        }
        out
    }
}

/// Returns the number of base-10 digits in `value`, counting zero as one digit.
fn digit_count(value: &BigUint) -> i64 {
    if value.is_zero() {
        return 1;
    }
    value.to_str_radix(10).len() as i64
}

/// Returns ten raised to `power`, treating non-positive powers as one.
fn pow10(power: i64) -> BigUint {
    if power <= 0 {
        BigUint::one()
    } else {
        BigUint::from(10u32).pow(power as u32)
    }
}

impl fmt::Display for Number {
    /// Formats the number using plain decimal notation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_plain_string())
    }
}

impl fmt::Debug for Number {
    /// Formats the number using plain decimal notation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_plain_string())
    }
}

impl PartialEq for Number {
    /// Compares numeric values, ignoring representation differences.
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    /// Compares numeric values.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl Ord for Number {
    /// Orders numeric values.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }
        if self.negative != other.negative {
            return if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        let exp = self.exp.min(other.exp);
        let a = &self.coeff * pow10(self.exp - exp);
        let b = &other.coeff * pow10(other.exp - exp);
        let ord = a.cmp(&b);
        if self.negative {
            ord.reverse()
        } else {
            ord
        }
    }
}
