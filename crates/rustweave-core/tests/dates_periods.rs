// SPDX-License-Identifier: MIT OR Apache-2.0
//! Behaviour tests for dates, periods, and date helpers.

fn run(script: &str) -> String {
    rustweave_core::run(script, &[]).unwrap_or_else(|error| panic!("{error}"))
}

const CASES: &[(&str, &str)] = &[
    (
        r####"{ d: |2018-06-19|, dt: |2018-06-19T14:45:30-05:00|, ldt: |2018-06-19T14:45:30|, lt: |14:45:30|, t: |14:45:30-05:00|, z: |-05:00|, p: |P2Y3M4D|, du: |PT2H15M| }"####,
        r####"{
  "d": "2018-06-19",
  "dt": "2018-06-19T14:45:30-05:00",
  "ldt": "2018-06-19T14:45:30",
  "lt": "14:45:30",
  "t": "14:45:30-05:00",
  "z": "-05:00",
  "p": "P2Y3M4D",
  "du": "PT2H15M"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Dates
output application/json
---
{
  date: date({year: 2018, month: 6, day: 19}),
  dateTime: dateTime({year: 2018, month: 6, day: 19, hour: 14, minutes: 45, seconds: 30, timeZone: |-05:00|}),
  localDateTime: localDateTime({year: 2018, month: 6, day: 19, hour: 14, minutes: 45, seconds: 30}),
  localTime: localTime({hour: 14, minutes: 45, seconds: 30}),
  time: time({hour: 14, minutes: 45, seconds: 30, timeZone: |-05:00|}),
  day: atBeginningOfDay(|2021-01-30T09:15:20.500-05:00|),
  hour: atBeginningOfHour(|09:15:20.500|),
  month: atBeginningOfMonth(|2021-01-30|),
  week: atBeginningOfWeek(|2021-01-30|),
  year: atBeginningOfYear(|2021-01-30|)
}"####,
        r####"{
  "date": "2018-06-19",
  "dateTime": "2018-06-19T14:45:30-05:00",
  "localDateTime": "2018-06-19T14:45:30",
  "localTime": "14:45:30",
  "time": "14:45:30-05:00",
  "day": "2021-01-30T00:00:00-05:00",
  "hour": "09:00:00",
  "month": "2021-01-01",
  "week": "2021-01-24",
  "year": "2021-01-01"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Periods
output application/json
---
{
  a: between(|2014-11-20|, |2014-11-18|),
  b: between(|2015-10-21|, |2014-09-19|),
  c: between(|2021-02-28|, |2021-03-31|),
  d: days(3),
  e: days(3.75),
  f: hours(3.75),
  g: minutes(3.75),
  h: seconds(3.75),
  i: months(3),
  j: years(3),
  k: duration({}),
  l: period({}),
  m: duration({days: 3, hours: 7, minutes: 42}),
  n: period({years: 3, months: 7, days: 42})
}"####,
        r####"{
  "a": "P2D",
  "b": "P1Y1M2D",
  "c": "P-1M-3D",
  "d": "P3D",
  "e": "PT90H",
  "f": "PT3H45M",
  "g": "PT3M45S",
  "h": "PT3.75S",
  "i": "P3M",
  "j": "P3Y",
  "k": "PT0S",
  "l": "P0D",
  "m": "PT79H42M",
  "n": "P3Y7M42D"
}"####,
    ),
    (
        r####"%dw 2.0
import * from dw::core::Periods
output application/json
---
{
  tomorrow: |2021-01-30T09:15:20.500Z| + days(1),
  yesterday: |2021-01-30T09:15:20.500Z| - days(1),
  quarter: |2021-01-30T00:00:00.000Z| + days(0.25),
  before: |2021-01-30T09:15:20.500Z| - duration({days: 2, hours: 3}),
  nextMonth: |2021-01-30T09:15:20.500Z| + months(1),
  nextYear: |2021-01-30T09:15:20.500Z| + years(1),
  hours: duration({days: 3, hours: 7, minutes: 42}).hours,
  months: period({years: 3, months: 7, days: 42}).months
}"####,
        r####"{
  "tomorrow": "2021-01-31T09:15:20.5Z",
  "yesterday": "2021-01-29T09:15:20.5Z",
  "quarter": "2021-01-30T06:00:00Z",
  "before": "2021-01-28T06:15:20.5Z",
  "nextMonth": "2021-02-28T09:15:20.5Z",
  "nextYear": "2022-01-30T09:15:20.5Z",
  "hours": 7,
  "months": 7
}"####,
    ),
    (
        r####"{
  days: daysBetween(|2020-02-29|, |2021-03-01|),
  back: daysBetween(|2021-03-01|, |2020-02-29|),
  leap: [isLeapYear(|2020-02-29|), isLeapYear(|2021-02-28|), isLeapYear(|2000-01-01|), isLeapYear(|1900-01-01|)]
}"####,
        r####"{
  "days": 366,
  "back": -366,
  "leap": [
    true,
    false,
    true,
    false
  ]
}"####,
    ),
    (
        r####"{
  year: |2018-06-19T14:45:30.250-05:00|.year,
  month: |2018-06-19T14:45:30.250-05:00|.month,
  day: |2018-06-19T14:45:30.250-05:00|.day,
  hour: |2018-06-19T14:45:30.250-05:00|.hour,
  minutes: |2018-06-19T14:45:30.250-05:00|.minutes,
  seconds: |2018-06-19T14:45:30.250-05:00|.seconds,
  milliseconds: |2018-06-19T14:45:30.250-05:00|.milliseconds,
  nanoseconds: |2018-06-19T14:45:30.250-05:00|.nanoseconds,
  quarter: |2018-06-19T14:45:30.250-05:00|.quarter,
  dayOfWeek: |2018-06-19T14:45:30.250-05:00|.dayOfWeek,
  dayOfYear: |2018-06-19T14:45:30.250-05:00|.dayOfYear,
  offsetSeconds: |2018-06-19T14:45:30.250-05:00|.offsetSeconds,
  years: |P2Y15M| as Number {unit: "years"},
  months: |P8Y15M| as Number {unit: "months"}
}"####,
        r####"{
  "year": 2018,
  "month": 6,
  "day": 19,
  "hour": 14,
  "minutes": 45,
  "seconds": 30,
  "milliseconds": 250,
  "nanoseconds": 250000000,
  "quarter": 2,
  "dayOfWeek": 2,
  "dayOfYear": 170,
  "offsetSeconds": -18000,
  "years": 3.25,
  "months": 111
}"####,
    ),
    (
        r####"{
  a: |2021-06-15T18:05:09| as String {format: "uuuu-MM-dd"},
  b: |2021-06-15T18:05:09| as String {format: "KK:mm:ss a"},
  c: |2021-06-15T18:05:09| as String {format: "KK:mm:ss a, MMMM dd, uuuu"},
  d: "2018/06/19" as Date {format: "uuuu/MM/dd"},
  e: "2018-06-19 14:45:30Z" as DateTime {format: "uuuu-MM-dd HH:mm:ssz"},
  f: "2018-06-19 14:45:30" as LocalDateTime {format: "uuuu-MM-dd HH:mm:ss"},
  g: "08:15:30.250" as LocalTime {format: "HH:mm:ss.n"}
}"####,
        r####"{
  "a": "2021-06-15",
  "b": "06:05:09 pm",
  "c": "06:05:09 pm, June 15, 2021",
  "d": "2018/06/19",
  "e": "2018-06-19 14:45:30Z",
  "f": "2018-06-19 14:45:30",
  "g": "08:15:30.250"
}"####,
    ),
    (
        r####"{
  before: |2018-06-19| < |2018-07-01|,
  same: |2018-06-19| == |2018-06-19|,
  ldt: |2019-03-21| ++ |08:14:02|,
  dt: |2019-03-21| ++ |08:14:02-05:00|
}"####,
        r####"{
  "before": true,
  "same": true,
  "ldt": "2019-03-21T08:14:02",
  "dt": "2019-03-21T08:14:02-05:00"
}"####,
    ),
];

const ERROR_CASES: &[&str] = &[
    r####"|2021-02-29|"####,
    r####"date({year: 2021, month: 14, day: 1})"####,
    r####"period({days: 2.5})"####,
];

#[test]
fn behaviour_cases() {
    for (script, expected) in CASES {
        assert_eq!(run(script), *expected, "script: {script}");
    }
    for script in ERROR_CASES {
        assert!(
            rustweave_core::run(script, &[]).is_err(),
            "expected error: {script}"
        );
    }
}
