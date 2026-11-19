// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for dates, periods, and date helpers.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

#[test]
fn temporal_literals_and_output() {
    let script = r#"{ d: |2012-10-11|, dt: |2012-10-11T12:30:40-03:00|, ldt: |2012-10-11T12:30:40|, lt: |12:30:40|, t: |12:30:40-03:00|, z: |-03:00|, p: |P1Y2M3D|, du: |PT1H30M| }"#;
    let expected = r#"{
  "d": "2012-10-11",
  "dt": "2012-10-11T12:30:40-03:00",
  "ldt": "2012-10-11T12:30:40",
  "lt": "12:30:40",
  "t": "12:30:40-03:00",
  "z": "-03:00",
  "p": "P1Y2M3D",
  "du": "PT1H30M"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn date_factories_and_beginning_functions() {
    let script = r#"%dw 2.0
import * from dw::core::Dates
output application/json
---
{
  date: date({year: 2012, month: 10, day: 11}),
  dateTime: dateTime({year: 2012, month: 10, day: 11, hour: 12, minutes: 30, seconds: 40, timeZone: |-03:00|}),
  localDateTime: localDateTime({year: 2012, month: 10, day: 11, hour: 12, minutes: 30, seconds: 40}),
  localTime: localTime({hour: 12, minutes: 30, seconds: 40}),
  time: time({hour: 12, minutes: 30, seconds: 40, timeZone: |-03:00|}),
  day: atBeginningOfDay(|2020-10-06T18:23:20.351-03:00|),
  hour: atBeginningOfHour(|18:23:20.351|),
  month: atBeginningOfMonth(|2020-10-06|),
  week: atBeginningOfWeek(|2020-10-06|),
  year: atBeginningOfYear(|2020-10-06|)
}"#;
    let expected = r#"{
  "date": "2012-10-11",
  "dateTime": "2012-10-11T12:30:40-03:00",
  "localDateTime": "2012-10-11T12:30:40",
  "localTime": "12:30:40",
  "time": "12:30:40-03:00",
  "day": "2020-10-06T00:00:00-03:00",
  "hour": "18:00:00",
  "month": "2020-10-01",
  "week": "2020-10-04",
  "year": "2020-01-01"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn period_factories() {
    let script = r#"%dw 2.0
import * from dw::core::Periods
output application/json
---
{
  a: between(|2010-12-12|, |2010-12-10|),
  b: between(|2011-12-11|, |2010-11-10|),
  c: between(|2020-02-29|, |2020-03-30|),
  d: days(4),
  e: days(4.555),
  f: hours(4.555),
  g: minutes(4.555),
  h: seconds(4.555),
  i: months(4),
  j: years(4),
  k: duration({}),
  l: period({}),
  m: duration({days: 4, hours: 11, minutes: 28}),
  n: period({years: 4, months: 11, days: 28})
}"#;
    let expected = r#"{
  "a": "P2D",
  "b": "P1Y1M1D",
  "c": "P-1M-1D",
  "d": "P4D",
  "e": "PT109H19M12S",
  "f": "PT4H33M18S",
  "g": "PT4M33.3S",
  "h": "PT4.555S",
  "i": "P4M",
  "j": "P4Y",
  "k": "PT0S",
  "l": "P0D",
  "m": "PT107H28M",
  "n": "P4Y11M28D"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn period_arithmetic_and_selectors() {
    let script = r#"%dw 2.0
import * from dw::core::Periods
output application/json
---
{
  tomorrow: |2020-10-05T20:22:34.385Z| + days(1),
  yesterday: |2020-10-05T20:22:34.385Z| - days(1),
  quarter: |2020-10-05T00:00:00.000Z| + days(0.25),
  before: |2020-10-05T20:22:34.385Z| - duration({days: 1, hours: 1}),
  nextMonth: |2020-10-05T20:22:34.385Z| + months(1),
  nextYear: |2020-10-05T20:22:34.385Z| + years(1),
  hours: duration({days: 4, hours: 11, minutes: 28}).hours,
  months: period({years: 4, months: 11, days: 28}).months
}"#;
    let expected = r#"{
  "tomorrow": "2020-10-06T20:22:34.385Z",
  "yesterday": "2020-10-04T20:22:34.385Z",
  "quarter": "2020-10-05T06:00:00Z",
  "before": "2020-10-04T19:22:34.385Z",
  "nextMonth": "2020-11-05T20:22:34.385Z",
  "nextYear": "2021-10-05T20:22:34.385Z",
  "hours": 11,
  "months": 11
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn core_date_helpers() {
    let script = r#"{
  days: daysBetween(|2016-10-01|, |2017-10-01|),
  back: daysBetween(|2017-10-01|, |2016-10-01|),
  leap: [isLeapYear(|2016-10-01|), isLeapYear(|2017-10-01|), isLeapYear(|2000-01-01|), isLeapYear(|1900-01-01|)]
}"#;
    let expected = r#"{
  "days": 365,
  "back": -365,
  "leap": [
    true,
    false,
    true,
    false
  ]
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn date_selectors_and_coercions() {
    let script = r#"{
  year: |2003-10-01T23:57:59.700-03:00|.year,
  month: |2003-10-01T23:57:59.700-03:00|.month,
  day: |2003-10-01T23:57:59.700-03:00|.day,
  hour: |2003-10-01T23:57:59.700-03:00|.hour,
  minutes: |2003-10-01T23:57:59.700-03:00|.minutes,
  seconds: |2003-10-01T23:57:59.700-03:00|.seconds,
  milliseconds: |2003-10-01T23:57:59.700-03:00|.milliseconds,
  nanoseconds: |2003-10-01T23:57:59.700-03:00|.nanoseconds,
  quarter: |2003-10-01T23:57:59.700-03:00|.quarter,
  dayOfWeek: |2003-10-01T23:57:59.700-03:00|.dayOfWeek,
  dayOfYear: |2003-10-01T23:57:59.700-03:00|.dayOfYear,
  offsetSeconds: |2003-10-01T23:57:59.700-03:00|.offsetSeconds,
  years: |P1Y12M| as Number {unit: "years"},
  months: |P8Y12M| as Number {unit: "months"}
}"#;
    let expected = r#"{
  "year": 2003,
  "month": 10,
  "day": 1,
  "hour": 23,
  "minutes": 57,
  "seconds": 59,
  "milliseconds": 700,
  "nanoseconds": 700000000,
  "quarter": 4,
  "dayOfWeek": 3,
  "dayOfYear": 274,
  "offsetSeconds": -10800,
  "years": 2,
  "months": 108
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn date_formats_and_parsing() {
    let script = r#"{
  a: |2020-10-01T23:57:59| as String {format: "uuuu-MM-dd"},
  b: |2020-10-01T23:57:59| as String {format: "KK:mm:ss a"},
  c: |2020-10-01T23:57:59| as String {format: "KK:mm:ss a, MMMM dd, uuuu"},
  d: "2003/10/01" as Date {format: "uuuu/MM/dd"},
  e: "2003-10-01 23:57:59Z" as DateTime {format: "uuuu-MM-dd HH:mm:ssz"},
  f: "2003-10-01 23:57:59" as LocalDateTime {format: "uuuu-MM-dd HH:mm:ss"},
  g: "13:44:12.283" as LocalTime {format: "HH:mm:ss.n"}
}"#;
    let expected = r#"{
  "a": "2020-10-01",
  "b": "11:57:59 PM",
  "c": "11:57:59 PM, October 01, 2020",
  "d": "2003-10-01",
  "e": "2003-10-01T23:57:59Z",
  "f": "2003-10-01T23:57:59",
  "g": "13:44:12.283"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn date_comparison_and_concatenation() {
    let script = r#"{
  before: |2003-10-01| < |2003-11-01|,
  same: |2003-10-01| == |2003-10-01|,
  ldt: |2017-10-01| ++ |23:57:59|,
  dt: |2017-10-01| ++ |23:57:59-03:00|
}"#;
    let expected = r#"{
  "before": true,
  "same": true,
  "ldt": "2017-10-01T23:57:59",
  "dt": "2017-10-01T23:57:59-03:00"
}"#;
    assert_eq!(run(script), expected);
}

#[test]
fn invalid_temporal_values_return_errors() {
    assert!(rustweave_core::run("|2019-02-30|", &[]).is_err());
    assert!(rustweave_core::run("date({year: 2020, month: 13, day: 1})", &[]).is_err());
    assert!(rustweave_core::run("period({days: 1.5})", &[]).is_err());
}
