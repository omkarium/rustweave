// SPDX-License-Identifier: MIT OR Apache-2.0
//! Date, time, time-zone, period, and binary value helpers.

use chrono::{
    Datelike, Duration as ChronoDuration, FixedOffset, Local, NaiveDate, NaiveDateTime,
    TimeZone as ChronoTimeZone, Timelike, Utc,
};
use num_traits::ToPrimitive;

use crate::error::DwError;
use crate::number::Number;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalTime {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub nano: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Zone {
    Offset { seconds: i32, zulu: bool },
    Named(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Time {
    pub time: LocalTime,
    pub zone: Zone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalDateTime {
    pub date: Date,
    pub time: LocalTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTime {
    pub date: Date,
    pub time: LocalTime,
    pub zone: Zone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Period {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub time_seconds: Number,
    pub date_based: bool,
}

impl Zone {
    pub fn utc() -> Self {
        Zone::Offset {
            seconds: 0,
            zulu: true,
        }
    }

    pub fn offset(seconds: i32) -> Self {
        Zone::Offset {
            seconds,
            zulu: seconds == 0,
        }
    }

    pub fn seconds(&self) -> i32 {
        match self {
            Zone::Offset { seconds, .. } => *seconds,
            Zone::Named(name) => named_zone_offset(name).unwrap_or(0),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Zone::Offset { seconds, zulu } => {
                if *zulu {
                    "Z".to_string()
                } else {
                    format_offset(*seconds)
                }
            }
            Zone::Named(name) => name.clone(),
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.eq_ignore_ascii_case("z") || text.eq_ignore_ascii_case("utc") {
            return Some(Zone::utc());
        }
        if text.eq_ignore_ascii_case("gmt") {
            return Some(Zone::Offset {
                seconds: 0,
                zulu: false,
            });
        }
        if let Some(seconds) = parse_offset(text) {
            return Some(Zone::Offset {
                seconds,
                zulu: false,
            });
        }
        if text.contains('/') || text.chars().all(|c| c.is_ascii_alphabetic()) {
            return Some(Zone::Named(text.to_string()));
        }
        None
    }
}

fn named_zone_offset(name: &str) -> Option<i32> {
    match name {
        "UTC" | "GMT" | "Etc/UTC" | "Etc/GMT" => Some(0),
        "America/New_York" | "America/Toronto" | "America/Detroit" => Some(-5 * 3600),
        "America/Chicago" | "America/Mexico_City" => Some(-6 * 3600),
        "America/Denver" | "America/Edmonton" => Some(-7 * 3600),
        "America/Los_Angeles" | "America/Vancouver" => Some(-8 * 3600),
        "America/Argentina/Buenos_Aires" | "America/Sao_Paulo" => Some(-3 * 3600),
        "Europe/London" => Some(0),
        "Europe/Paris" | "Europe/Berlin" | "Europe/Madrid" | "Europe/Rome" => Some(3600),
        "Asia/Tokyo" | "Asia/Seoul" => Some(9 * 3600),
        "Asia/Kolkata" | "Asia/Calcutta" => Some(5 * 3600 + 1800),
        "Australia/Sydney" | "Australia/Melbourne" => Some(10 * 3600),
        _ => None,
    }
}

fn parse_offset(text: &str) -> Option<i32> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let sign = match bytes[0] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let rest = &text[1..];
    let (hour, minute) = if let Some((h, m)) = rest.split_once(':') {
        (h, m)
    } else if rest.len() == 4 {
        (&rest[..2], &rest[2..])
    } else if rest.len() == 2 {
        (rest, "0")
    } else {
        return None;
    };
    let hour: i32 = hour.parse().ok()?;
    let minute: i32 = minute.parse().ok()?;
    if hour > 23 || minute > 59 {
        return None;
    }
    Some(sign * (hour * 3600 + minute * 60))
}

pub fn format_offset(seconds: i32) -> String {
    let sign = if seconds < 0 { '-' } else { '+' };
    let total = seconds.abs();
    format!("{}{:02}:{:02}", sign, total / 3600, (total % 3600) / 60)
}

fn offset_zone(zone: &Zone) -> FixedOffset {
    FixedOffset::east_opt(zone.seconds()).unwrap_or_else(|| FixedOffset::east_opt(0).expect("zero"))
}

fn naive_date(date: &Date) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(date.year, date.month, date.day)
}

fn date_from_naive(date: NaiveDate) -> Date {
    Date {
        year: date.year(),
        month: date.month(),
        day: date.day(),
    }
}

fn local_time_from_naive(time: chrono::NaiveTime) -> LocalTime {
    LocalTime {
        hour: time.hour(),
        minute: time.minute(),
        second: time.second(),
        nano: time.nanosecond(),
    }
}

fn naive_time(time: &LocalTime) -> Option<chrono::NaiveTime> {
    chrono::NaiveTime::from_hms_nano_opt(time.hour, time.minute, time.second, time.nano)
}

fn naive_local(date: &LocalDateTime) -> Option<NaiveDateTime> {
    Some(NaiveDateTime::new(
        naive_date(&date.date)?,
        naive_time(&date.time)?,
    ))
}

fn date_time_utc(date: &DateTime) -> Option<NaiveDateTime> {
    let local = NaiveDateTime::new(naive_date(&date.date)?, naive_time(&date.time)?);
    Some(local - ChronoDuration::seconds(date.zone.seconds() as i64))
}

fn fraction_text(nano: u32) -> String {
    if nano == 0 {
        return String::new();
    }
    let mut text = format!("{nano:09}");
    while text.ends_with('0') {
        text.pop();
    }
    format!(".{text}")
}

pub fn parse_fraction(text: &str) -> Option<u32> {
    if text.is_empty() {
        return Some(0);
    }
    let digits: String = text.chars().take(9).collect();
    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let mut padded = digits;
    while padded.len() < 9 {
        padded.push('0');
    }
    padded.parse().ok()
}

pub fn format_date(date: &Date) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

pub fn format_local_time(time: &LocalTime) -> String {
    format!(
        "{:02}:{:02}:{:02}{}",
        time.hour,
        time.minute,
        time.second,
        fraction_text(time.nano)
    )
}

pub fn format_time(time: &Time) -> String {
    format!("{}{}", format_local_time(&time.time), time.zone.label())
}

pub fn format_local_date_time(date: &LocalDateTime) -> String {
    format!(
        "{}T{}",
        format_date(&date.date),
        format_local_time(&date.time)
    )
}

pub fn format_date_time(date: &DateTime) -> String {
    format!(
        "{}T{}{}",
        format_date(&date.date),
        format_local_time(&date.time),
        date.zone.label()
    )
}

pub fn format_period(period: &Period) -> String {
    let has_date = period.years != 0 || period.months != 0 || period.days != 0;
    let mut out = String::from("P");
    if period.years != 0 {
        out.push_str(&period.years.to_string());
        out.push('Y');
    }
    if period.months != 0 {
        out.push_str(&period.months.to_string());
        out.push('M');
    }
    if period.days != 0 {
        out.push_str(&period.days.to_string());
        out.push('D');
    }
    let seconds = period.time_seconds.clone();
    let zero_seconds = seconds.is_zero();
    if !zero_seconds {
        out.push('T');
        let negative = seconds.is_negative();
        let text = seconds.abs().to_string();
        let (whole_text, fraction) = match text.split_once('.') {
            Some((whole, fraction)) => (whole, Some(fraction)),
            None => (text.as_str(), None),
        };
        let whole: i64 = whole_text.parse().unwrap_or(0);
        let hours = whole / 3_600;
        let minutes = (whole % 3_600) / 60;
        let secs = whole % 60;
        let sign = if negative { "-" } else { "" };
        if hours != 0 {
            out.push_str(&format!("{sign}{hours}H"));
        }
        if minutes != 0 {
            out.push_str(&format!("{sign}{minutes}M"));
        }
        if secs != 0 || fraction.is_some() {
            match fraction {
                Some(fraction) => out.push_str(&format!("{sign}{secs}.{fraction}S")),
                None => out.push_str(&format!("{sign}{secs}S")),
            }
        }
        if out.ends_with('T') {
            out.push_str("0S");
        }
    } else if !has_date {
        if period.date_based {
            out.push_str("0D");
        } else {
            out.push_str("T0S");
        }
    }
    out
}

pub fn parse_literal(text: &str) -> Option<crate::value::Value> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if let Some(period) = parse_period(text) {
        return Some(crate::value::Value::Period(period));
    }
    if text == "Z" || text == "z" {
        return Some(crate::value::Value::TimeZone(Zone::utc()));
    }
    if (text.starts_with('+') || text.starts_with('-'))
        && text.matches(':').count() == 1
        && !text.contains('T')
    {
        if let Some(zone) = Zone::parse(text) {
            return Some(crate::value::Value::TimeZone(zone));
        }
    }
    if let Some(value) = parse_date_literal(text) {
        return Some(value);
    }
    if let Some(value) = parse_time_literal(text) {
        return Some(value);
    }
    if text.len() % 2 == 0 && text.chars().all(|c| c.is_ascii_hexdigit()) {
        return Some(crate::value::Value::Binary(hex_decode(text).ok()?));
    }
    None
}

fn parse_date_literal(text: &str) -> Option<crate::value::Value> {
    if let Some((date_part, rest)) = text.split_once('T') {
        let date = parse_date(date_part)?;
        let (time_part, zone) = split_zone(rest);
        let time = parse_time_of_day(time_part)?;
        return Some(match zone {
            Some(zone) => crate::value::Value::DateTime(DateTime { date, time, zone }),
            None => crate::value::Value::LocalDateTime(LocalDateTime { date, time }),
        });
    }
    parse_date(text).map(crate::value::Value::Date)
}

fn parse_time_literal(text: &str) -> Option<crate::value::Value> {
    let (time_part, zone) = split_zone(text);
    if time_part.matches(':').count() < 1 {
        return None;
    }
    let time = parse_time_of_day(time_part)?;
    Some(match zone {
        Some(zone) => crate::value::Value::Time(Time { time, zone }),
        None => crate::value::Value::LocalTime(time),
    })
}

fn split_zone(text: &str) -> (&str, Option<Zone>) {
    if let Some(stripped) = text.strip_suffix('Z') {
        if !stripped.is_empty() {
            return (stripped, Some(Zone::utc()));
        }
    }
    if let Some(stripped) = text.strip_suffix('z') {
        if !stripped.is_empty() {
            return (stripped, Some(Zone::utc()));
        }
    }
    if text.len() > 6 {
        let split = text.len() - 6;
        if let Some(candidate) = text.get(split..) {
            if (candidate.starts_with('+') || candidate.starts_with('-'))
                && candidate.as_bytes().get(3) == Some(&b':')
            {
                if let Some(zone) = Zone::parse(candidate) {
                    return (&text[..split], Some(zone));
                }
            }
        }
    }
    if text.len() > 5 {
        let split = text.len() - 5;
        if let Some(candidate) = text.get(split..) {
            if (candidate.starts_with('+') || candidate.starts_with('-'))
                && candidate[1..].chars().all(|c| c.is_ascii_digit())
            {
                if let Some(zone) = Zone::parse(candidate) {
                    return (&text[..split], Some(zone));
                }
            }
        }
    }
    (text, None)
}

pub fn parse_date(text: &str) -> Option<Date> {
    let mut parts = text.split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    NaiveDate::from_ymd_opt(year, month, day)?;
    Some(Date { year, month, day })
}

pub fn parse_time_of_day(text: &str) -> Option<LocalTime> {
    let mut parts = text.split(':');
    let hour: u32 = parts.next()?.parse().ok()?;
    let minute: u32 = parts.next()?.parse().ok()?;
    let second_part = parts.next().unwrap_or("0");
    if parts.next().is_some() {
        return None;
    }
    let (second_text, fraction) = match second_part.split_once('.') {
        Some((second, fraction)) => (second, Some(fraction)),
        None => (second_part, None),
    };
    let second: u32 = second_text.parse().ok()?;
    let nano = parse_fraction(fraction.unwrap_or(""))?;
    chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano)?;
    Some(LocalTime {
        hour,
        minute,
        second,
        nano,
    })
}

pub fn parse_period(text: &str) -> Option<Period> {
    let mut rest = text;
    let negative = if let Some(stripped) = rest.strip_prefix('-') {
        rest = stripped;
        true
    } else {
        false
    };
    if !rest.starts_with('P') {
        return None;
    }
    rest = &rest[1..];
    let mut years = 0i64;
    let mut months = 0i64;
    let mut days = 0i64;
    let mut time_seconds = Number::zero();
    let mut in_time = false;
    let mut saw_date = false;
    let mut saw_time = false;
    let mut number = String::new();
    for c in rest.chars() {
        match c {
            'T' => {
                in_time = true;
                number.clear();
            }
            '0'..='9' | '.' | '-' | '+' => number.push(c),
            'Y' | 'M' | 'D' | 'H' | 'S' => {
                if number.is_empty() || number == "-" || number == "+" {
                    return None;
                }
                if c == 'M' && in_time {
                    let value: f64 = number.parse().ok()?;
                    let seconds = Number::from_f64(value * 60.0)?;
                    time_seconds = time_seconds.add(&seconds);
                    saw_time = true;
                } else if c == 'M' {
                    months = number.parse().ok()?;
                    saw_date = true;
                } else {
                    let value: f64 = number.parse().ok()?;
                    match c {
                        'Y' => {
                            years = number.parse().ok()?;
                            saw_date = true;
                        }
                        'D' => {
                            days = number.parse().ok()?;
                            saw_date = true;
                        }
                        'H' => {
                            let seconds = Number::from_f64(value * 3600.0)?;
                            time_seconds = time_seconds.add(&seconds);
                            saw_time = true;
                        }
                        'S' => {
                            let seconds = Number::from_f64(value)?;
                            time_seconds = time_seconds.add(&seconds);
                            saw_time = true;
                        }
                        _ => {}
                    }
                }
                number.clear();
            }
            _ => return None,
        }
    }
    if rest.is_empty() || (!saw_date && !saw_time) {
        if rest == "P" {
            return Some(Period {
                years: 0,
                months: 0,
                days: 0,
                time_seconds: Number::zero(),
                date_based: false,
            });
        }
        return None;
    }
    if negative {
        years = -years;
        months = -months;
        days = -days;
        time_seconds = time_seconds.neg();
    }
    Some(Period {
        years,
        months,
        days,
        time_seconds,
        date_based: saw_date,
    })
}

pub fn date_period(years: i64, months: i64, days: i64) -> Period {
    Period {
        years,
        months,
        days,
        time_seconds: Number::zero(),
        date_based: true,
    }
}

pub fn duration_period(seconds: Number) -> Period {
    Period {
        years: 0,
        months: 0,
        days: 0,
        time_seconds: seconds,
        date_based: false,
    }
}

pub fn number_to_i64(number: &Number) -> Result<i64, DwError> {
    number
        .to_i64()
        .ok_or_else(|| DwError::new("expected a whole number"))
}

pub fn number_to_f64(number: &Number) -> Result<f64, DwError> {
    number
        .to_f64()
        .ok_or_else(|| DwError::new("number is out of range"))
}

fn last_day_of_month(year: i32, month: u32) -> Option<u32> {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first = NaiveDate::from_ymd_opt(next_year, next_month, 1)?;
    Some((first - ChronoDuration::days(1)).day())
}

fn add_months(date: &Date, months: i64) -> Option<Date> {
    let total = date.year as i64 * 12 + date.month as i64 - 1 + months;
    let year = total.div_euclid(12) as i32;
    let month = (total.rem_euclid(12) + 1) as u32;
    let day = date.day.min(last_day_of_month(year, month)?);
    NaiveDate::from_ymd_opt(year, month, day)?;
    Some(Date { year, month, day })
}

pub fn add_period_to_date(date: &Date, period: &Period) -> Result<Date, DwError> {
    let mut result = *date;
    result = add_months(&result, period.years * 12 + period.months)
        .ok_or_else(|| DwError::new("date out of range"))?;
    let base = NaiveDate::from_ymd_opt(result.year, result.month, result.day)
        .ok_or_else(|| DwError::new("date out of range"))?;
    let shifted = base + ChronoDuration::days(period.days);
    Ok(date_from_naive(shifted))
}

pub fn add_period_to_local_date_time(
    value: &LocalDateTime,
    period: &Period,
) -> Result<LocalDateTime, DwError> {
    let date = add_period_to_date(&value.date, period)?;
    let base = NaiveDateTime::new(
        naive_date(&date).ok_or_else(|| DwError::new("date out of range"))?,
        naive_time(&value.time).ok_or_else(|| DwError::new("time out of range"))?,
    );
    let seconds = number_to_f64(&period.time_seconds)?;
    let nanos = (seconds * 1_000_000_000.0).round();
    if !nanos.is_finite() || nanos.abs() > i64::MAX as f64 {
        return Err(DwError::new("duration out of range"));
    }
    let shifted = base + ChronoDuration::nanoseconds(nanos as i64);
    Ok(LocalDateTime {
        date: date_from_naive(shifted.date()),
        time: local_time_from_naive(shifted.time()),
    })
}

pub fn add_period_to_date_time(value: &DateTime, period: &Period) -> Result<DateTime, DwError> {
    let local = add_period_to_local_date_time(
        &LocalDateTime {
            date: value.date,
            time: value.time,
        },
        period,
    )?;
    Ok(DateTime {
        date: local.date,
        time: local.time,
        zone: value.zone.clone(),
    })
}

pub fn add_period_to_time(value: &Time, period: &Period) -> Result<Time, DwError> {
    let base = naive_local(&LocalDateTime {
        date: Date {
            year: 1970,
            month: 1,
            day: 1,
        },
        time: value.time,
    })
    .ok_or_else(|| DwError::new("time out of range"))?;
    let seconds = number_to_f64(&period.time_seconds)?;
    let nanos = (seconds * 1_000_000_000.0).round();
    if !nanos.is_finite() || nanos.abs() > i64::MAX as f64 {
        return Err(DwError::new("duration out of range"));
    }
    let shifted = base + ChronoDuration::nanoseconds(nanos as i64);
    Ok(Time {
        time: local_time_from_naive(shifted.time()),
        zone: value.zone.clone(),
    })
}

pub fn add_period_to_local_time(value: &LocalTime, period: &Period) -> Result<LocalTime, DwError> {
    let base = naive_local(&LocalDateTime {
        date: Date {
            year: 1970,
            month: 1,
            day: 1,
        },
        time: *value,
    })
    .ok_or_else(|| DwError::new("time out of range"))?;
    let seconds = number_to_f64(&period.time_seconds)?;
    let nanos = (seconds * 1_000_000_000.0).round();
    if !nanos.is_finite() || nanos.abs() > i64::MAX as f64 {
        return Err(DwError::new("duration out of range"));
    }
    let shifted = base + ChronoDuration::nanoseconds(nanos as i64);
    Ok(local_time_from_naive(shifted.time()))
}

pub fn subtract_period_from_date(date: &Date, period: &Period) -> Result<Date, DwError> {
    let mut negated = period.clone();
    negated.years = -negated.years;
    negated.months = -negated.months;
    negated.days = -negated.days;
    negated.time_seconds = negated.time_seconds.neg();
    add_period_to_date(date, &negated)
}

pub fn subtract_period_from_local_date_time(
    value: &LocalDateTime,
    period: &Period,
) -> Result<LocalDateTime, DwError> {
    let mut negated = period.clone();
    negated.years = -negated.years;
    negated.months = -negated.months;
    negated.days = -negated.days;
    negated.time_seconds = negated.time_seconds.neg();
    add_period_to_local_date_time(value, &negated)
}

pub fn subtract_period_from_date_time(
    value: &DateTime,
    period: &Period,
) -> Result<DateTime, DwError> {
    let mut negated = period.clone();
    negated.years = -negated.years;
    negated.months = -negated.months;
    negated.days = -negated.days;
    negated.time_seconds = negated.time_seconds.neg();
    add_period_to_date_time(value, &negated)
}

pub fn date_difference(start: &Date, end: &Date) -> Result<i64, DwError> {
    let start = NaiveDate::from_ymd_opt(start.year, start.month, start.day)
        .ok_or_else(|| DwError::new("date out of range"))?;
    let end = NaiveDate::from_ymd_opt(end.year, end.month, end.day)
        .ok_or_else(|| DwError::new("date out of range"))?;
    Ok((end - start).num_days())
}

pub fn local_date_time_difference(
    start: &LocalDateTime,
    end: &LocalDateTime,
) -> Result<Number, DwError> {
    let start = naive_local(start).ok_or_else(|| DwError::new("date out of range"))?;
    let end = naive_local(end).ok_or_else(|| DwError::new("date out of range"))?;
    let duration = end - start;
    let seconds = duration.num_seconds();
    let nanos = duration.subsec_nanos();
    let mut value = Number::from_i64(seconds);
    if nanos != 0 {
        let fraction = Number::from_i64(nanos as i64).div(&Number::from_i64(1_000_000_000));
        if let Some(fraction) = fraction {
            value = value.add(&fraction);
        }
    }
    Ok(value)
}

pub fn date_time_difference(start: &DateTime, end: &DateTime) -> Result<Number, DwError> {
    let start = date_time_utc(start).ok_or_else(|| DwError::new("date out of range"))?;
    let end = date_time_utc(end).ok_or_else(|| DwError::new("date out of range"))?;
    let duration = end - start;
    let seconds = duration.num_seconds();
    let nanos = duration.subsec_nanos();
    let mut value = Number::from_i64(seconds);
    if nanos != 0 {
        let fraction = Number::from_i64(nanos as i64).div(&Number::from_i64(1_000_000_000));
        if let Some(fraction) = fraction {
            value = value.add(&fraction);
        }
    }
    Ok(value)
}

pub fn period_between(start: &Date, end: &Date) -> Result<Period, DwError> {
    if start == end {
        return Ok(date_period(0, 0, 0));
    }
    if start > end {
        let mut value = period_between(end, start)?;
        value.years = -value.years;
        value.months = -value.months;
        value.days = -value.days;
        return Ok(value);
    }
    let end_date = NaiveDate::from_ymd_opt(end.year, end.month, end.day)
        .ok_or_else(|| DwError::new("date out of range"))?;
    let mut months =
        (end.year as i64 - start.year as i64) * 12 + end.month as i64 - start.month as i64;
    let mut candidate =
        add_months(start, months).ok_or_else(|| DwError::new("date out of range"))?;
    let mut candidate_date =
        NaiveDate::from_ymd_opt(candidate.year, candidate.month, candidate.day)
            .ok_or_else(|| DwError::new("date out of range"))?;
    if candidate_date > end_date {
        months -= 1;
        candidate = add_months(start, months).ok_or_else(|| DwError::new("date out of range"))?;
        candidate_date = NaiveDate::from_ymd_opt(candidate.year, candidate.month, candidate.day)
            .ok_or_else(|| DwError::new("date out of range"))?;
    }
    let days = (end_date - candidate_date).num_days();
    Ok(date_period(months / 12, months % 12, days))
}

impl Date {
    pub fn add_days(self, days: i64) -> Result<Date, DwError> {
        let date = NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .ok_or_else(|| DwError::new("date out of range"))?;
        Ok(date_from_naive(date + ChronoDuration::days(days)))
    }

    pub fn day_of_week(self) -> Result<u32, DwError> {
        let date = NaiveDate::from_ymd_opt(self.year, self.month, self.day)
            .ok_or_else(|| DwError::new("date out of range"))?;
        Ok(date.weekday().num_days_from_monday() + 1)
    }

    pub fn beginning_of_week(self) -> Result<Date, DwError> {
        let offset = self.day_of_week()? % 7;
        self.add_days(-(offset as i64))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.year, self.month, self.day).cmp(&(other.year, other.month, other.day)))
    }
}

impl PartialOrd for LocalTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.hour, self.minute, self.second, self.nano).cmp(&(
            other.hour,
            other.minute,
            other.second,
            other.nano,
        )))
    }
}

impl PartialOrd for LocalDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.date.partial_cmp(&other.date)? {
            std::cmp::Ordering::Equal => self.time.partial_cmp(&other.time),
            other => Some(other),
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        date_time_utc(self).partial_cmp(&date_time_utc(other))
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_total =
            self.time.hour as i64 * 3600 + self.time.minute as i64 * 60 + self.time.second as i64
                - self.zone.seconds() as i64;
        let other_total = other.time.hour as i64 * 3600
            + other.time.minute as i64 * 60
            + other.time.second as i64
            - other.zone.seconds() as i64;
        Some((self_total, self.time.nano).cmp(&(other_total, other.time.nano)))
    }
}

pub fn now() -> DateTime {
    let current = Local::now();
    let offset = current.offset().local_minus_utc();
    DateTime {
        date: date_from_naive(current.date_naive()),
        time: local_time_from_naive(current.time()),
        zone: Zone::Offset {
            seconds: offset,
            zulu: offset == 0,
        },
    }
}

pub fn today() -> Date {
    date_from_naive(Local::now().date_naive())
}

pub fn utc_date_time_from_timestamp(seconds: i64, millis: bool) -> Result<DateTime, DwError> {
    let timestamp = if millis { seconds / 1000 } else { seconds };
    let nanos = if millis {
        (seconds.rem_euclid(1000) * 1_000_000) as u32
    } else {
        0
    };
    let utc = Utc
        .timestamp_opt(timestamp, nanos)
        .single()
        .ok_or_else(|| DwError::new("timestamp out of range"))?;
    Ok(DateTime {
        date: date_from_naive(utc.date_naive()),
        time: local_time_from_naive(utc.time()),
        zone: Zone::utc(),
    })
}

pub fn timestamp_seconds(value: &DateTime) -> Result<i64, DwError> {
    let utc = date_time_utc(value).ok_or_else(|| DwError::new("date out of range"))?;
    Ok(utc.and_utc().timestamp())
}

pub fn month_name(month: u32, short: bool) -> &'static str {
    const FULL: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    const SHORT: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let index = (month.clamp(1, 12) - 1) as usize;
    if short {
        SHORT[index]
    } else {
        FULL[index]
    }
}

pub fn month_from_name(name: &str) -> Option<u32> {
    (1..=12).find(|&month| {
        name.eq_ignore_ascii_case(month_name(month, false))
            || name.eq_ignore_ascii_case(month_name(month, true))
    })
}

pub fn weekday_name(weekday: u32, short: bool) -> &'static str {
    const FULL: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    const SHORT: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let index = weekday.saturating_sub(1) as usize % 7;
    if short {
        SHORT[index]
    } else {
        FULL[index]
    }
}

pub fn weekday_from_name(name: &str) -> Option<u32> {
    (1..=7).find(|&day| {
        name.eq_ignore_ascii_case(weekday_name(day, false))
            || name.eq_ignore_ascii_case(weekday_name(day, true))
    })
}

pub fn hex_decode(text: &str) -> Result<Vec<u8>, DwError> {
    let text = text.trim();
    if text.len() % 2 != 0 {
        return Err(DwError::new("invalid hexadecimal string"));
    }
    let mut out = Vec::with_capacity(text.len() / 2);
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let high = hex_digit(bytes[index])?;
        let low = hex_digit(bytes[index + 1])?;
        out.push(high * 16 + low);
        index += 2;
    }
    Ok(out)
}

fn hex_digit(c: u8) -> Result<u8, DwError> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(DwError::new("invalid hexadecimal string")),
    }
}

pub fn hex_encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    out
}

pub fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let value = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((value >> 18) & 63) as usize] as char);
        out.push(TABLE[((value >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((value >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(value & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub fn base64_decode(text: &str) -> Result<Vec<u8>, DwError> {
    let mut values = Vec::new();
    for c in text.chars().filter(|c| !c.is_whitespace()) {
        if c == '=' {
            break;
        }
        let value = match c {
            'A'..='Z' => c as u8 - b'A',
            'a'..='z' => c as u8 - b'a' + 26,
            '0'..='9' => c as u8 - b'0' + 52,
            '+' => 62,
            '/' => 63,
            _ => return Err(DwError::new("invalid base64 string")),
        };
        values.push(value);
    }
    let mut out = Vec::new();
    for chunk in values.chunks(4) {
        match chunk.len() {
            4 => {
                let value = ((chunk[0] as u32) << 18)
                    | ((chunk[1] as u32) << 12)
                    | ((chunk[2] as u32) << 6)
                    | chunk[3] as u32;
                out.push((value >> 16) as u8);
                out.push((value >> 8) as u8);
                out.push(value as u8);
            }
            3 => {
                let value = ((chunk[0] as u32) << 18)
                    | ((chunk[1] as u32) << 12)
                    | ((chunk[2] as u32) << 6);
                out.push((value >> 16) as u8);
                out.push((value >> 8) as u8);
            }
            2 => {
                let value = ((chunk[0] as u32) << 18) | ((chunk[1] as u32) << 12);
                out.push((value >> 16) as u8);
            }
            _ => return Err(DwError::new("invalid base64 string")),
        }
    }
    Ok(out)
}

pub fn to_f64(number: &Number) -> Option<f64> {
    number.to_f64()
}

pub fn number_from_f64(value: f64) -> Result<Number, DwError> {
    Number::from_f64(value).ok_or_else(|| DwError::new("number is out of range"))
}

pub fn number_to_usize(number: &Number) -> Option<usize> {
    number
        .to_i64()
        .and_then(|value| usize::try_from(value).ok())
}

pub fn number_to_u32(number: &Number) -> Option<u32> {
    number.to_i64().and_then(|value| u32::try_from(value).ok())
}

pub fn number_to_i32(number: &Number) -> Option<i32> {
    number.to_i64().and_then(|value| i32::try_from(value).ok())
}

pub fn to_u64(number: &Number) -> Option<u64> {
    number.to_i64().and_then(|value| u64::try_from(value).ok())
}

pub fn to_i64_number(value: i64) -> Number {
    Number::from_i64(value)
}

pub fn f64_to_i64(value: f64) -> i64 {
    value as i64
}

pub fn f64_trunc(value: f64) -> f64 {
    value.trunc()
}

pub fn f64_abs(value: f64) -> f64 {
    value.abs()
}

pub fn f64_round(value: f64) -> f64 {
    value.round()
}

pub fn f64_floor(value: f64) -> f64 {
    value.floor()
}

pub fn f64_ceil(value: f64) -> f64 {
    value.ceil()
}

pub fn f64_sqrt(value: f64) -> f64 {
    value.sqrt()
}

pub fn f64_pow(value: f64, exponent: f64) -> f64 {
    value.powf(exponent)
}

pub fn f64_ln(value: f64) -> f64 {
    value.ln()
}

pub fn f64_log10(value: f64) -> f64 {
    value.log10()
}

pub fn f64_sin(value: f64) -> f64 {
    value.sin()
}

pub fn f64_cos(value: f64) -> f64 {
    value.cos()
}

pub fn f64_tan(value: f64) -> f64 {
    value.tan()
}

pub fn f64_asin(value: f64) -> f64 {
    value.asin()
}

pub fn f64_acos(value: f64) -> f64 {
    value.acos()
}

pub fn f64_atan(value: f64) -> f64 {
    value.atan()
}

pub fn f64_to_radians(value: f64) -> f64 {
    value.to_radians()
}

pub fn f64_to_degrees(value: f64) -> f64 {
    value.to_degrees()
}

pub fn f64_to_number_trunc(value: f64) -> Option<Number> {
    let _ = value.to_i64()?;
    Number::from_f64(value)
}

#[derive(Clone, Debug)]
enum PatternToken {
    Field(char, usize),
    Literal(String),
}

fn compile_pattern(pattern: &str) -> Vec<PatternToken> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let c = chars[index];
        if c == '\'' {
            index += 1;
            let mut literal = String::new();
            while index < chars.len() {
                if chars[index] == '\'' {
                    if chars.get(index + 1) == Some(&'\'') {
                        literal.push('\'');
                        index += 2;
                    } else {
                        index += 1;
                        break;
                    }
                } else {
                    literal.push(chars[index]);
                    index += 1;
                }
            }
            out.push(PatternToken::Literal(literal));
        } else if c == '\\' {
            index += 1;
            if let Some(next) = chars.get(index) {
                out.push(PatternToken::Literal(next.to_string()));
                index += 1;
            }
        } else if c.is_ascii_alphabetic() {
            let mut count = 0;
            while index < chars.len() && chars[index] == c {
                count += 1;
                index += 1;
            }
            out.push(PatternToken::Field(c, count));
        } else {
            out.push(PatternToken::Literal(c.to_string()));
            index += 1;
        }
    }
    out
}

fn pad_number(value: i64, count: usize) -> String {
    if count <= 1 {
        value.to_string()
    } else {
        format!("{:0width$}", value, width = count)
    }
}

fn week_of_year(date: &NaiveDate) -> u32 {
    date.iso_week().week()
}

fn week_of_month(date: &NaiveDate) -> u32 {
    (date.day() - 1) / 7 + 1
}

fn field_text(
    c: char,
    count: usize,
    date: Option<&NaiveDate>,
    time: Option<&chrono::NaiveTime>,
    zone: Option<&Zone>,
) -> String {
    match c {
        'G' => "AD".to_string(),
        'u' | 'y' => {
            let year = date.map(|d| d.year()).unwrap_or(1970) as i64;
            if count == 2 {
                format!("{:02}", year.rem_euclid(100))
            } else {
                pad_number(year, count.max(4))
            }
        }
        'Y' => {
            let year = date.map(|d| d.iso_week().year()).unwrap_or(1970) as i64;
            if count == 2 {
                format!("{:02}", year.rem_euclid(100))
            } else {
                pad_number(year, count.max(4))
            }
        }
        'M' | 'L' => {
            let month = date.map(|d| d.month()).unwrap_or(1);
            if count >= 4 {
                month_name(month, false).to_string()
            } else if count == 3 {
                month_name(month, true).to_string()
            } else {
                pad_number(month as i64, count)
            }
        }
        'd' => pad_number(date.map(|d| d.day()).unwrap_or(1) as i64, count),
        'D' => pad_number(date.map(|d| d.ordinal()).unwrap_or(1) as i64, count),
        'Q' | 'q' => {
            let quarter = date.map(|d| (d.month() - 1) / 3 + 1).unwrap_or(1);
            match (c, count) {
                ('Q', 4) => format!("{quarter}th quarter"),
                ('Q', 3) => format!("Q{quarter}"),
                _ => pad_number(quarter as i64, count),
            }
        }
        'E' | 'e' | 'c' => {
            let weekday = date
                .map(|d| d.weekday().num_days_from_monday() + 1)
                .unwrap_or(1);
            if count >= 4 {
                weekday_name(weekday, false).to_string()
            } else if count == 3 {
                weekday_name(weekday, true).to_string()
            } else if count == 2 {
                pad_number(weekday as i64, 2)
            } else {
                weekday.to_string()
            }
        }
        'w' => pad_number(date.map(week_of_year).unwrap_or(1) as i64, count),
        'W' => pad_number(date.map(week_of_month).unwrap_or(1) as i64, count),
        'F' => pad_number(date.map(week_of_month).unwrap_or(1) as i64, count),
        'H' => pad_number(time.map(|t| t.hour()).unwrap_or(0) as i64, count),
        'h' => {
            let hour = time.map(|t| t.hour()).unwrap_or(0);
            let hour = if hour % 12 == 0 { 12 } else { hour % 12 };
            pad_number(hour as i64, count)
        }
        'K' => pad_number(time.map(|t| t.hour() % 12).unwrap_or(0) as i64, count),
        'k' => {
            let hour = time.map(|t| t.hour()).unwrap_or(0);
            let hour = if hour == 0 { 24 } else { hour };
            pad_number(hour as i64, count)
        }
        'm' => pad_number(time.map(|t| t.minute()).unwrap_or(0) as i64, count),
        's' => pad_number(time.map(|t| t.second()).unwrap_or(0) as i64, count),
        'S' => {
            let nano = time.map(|t| t.nanosecond()).unwrap_or(0) as u64;
            let digits = count.clamp(1, 9);
            let divisor = 10u64.pow((9 - digits) as u32);
            format!("{:0width$}", nano / divisor, width = digits)
        }
        'n' => format!("{:09}", time.map(|t| t.nanosecond()).unwrap_or(0)),
        'N' => pad_number(
            time.map(|t| {
                t.num_seconds_from_midnight() as i64 * 1_000_000_000 + t.nanosecond() as i64
            })
            .unwrap_or(0),
            count.max(1),
        ),
        'A' => pad_number(
            time.map(|t| {
                t.num_seconds_from_midnight() as i64 * 1000 + (t.nanosecond() / 1_000_000) as i64
            })
            .unwrap_or(0),
            count.max(1),
        ),
        'a' => {
            let hour = time.map(|t| t.hour()).unwrap_or(0);
            if hour < 12 { "AM" } else { "PM" }.to_string()
        }
        'z' | 'Z' | 'X' | 'x' | 'O' | 'V' => {
            if let Some(zone) = zone {
                if let Zone::Named(name) = zone {
                    return name.clone();
                }
                let seconds = zone.seconds();
                match c {
                    'O' => {
                        if seconds == 0 {
                            "GMT".to_string()
                        } else {
                            let sign = if seconds < 0 { '-' } else { '+' };
                            let total = seconds.abs();
                            format!("GMT{}{}", sign, total / 3600)
                        }
                    }
                    'X' | 'x' if count <= 1 => {
                        if seconds == 0 && c == 'X' {
                            "Z".to_string()
                        } else {
                            let sign = if seconds < 0 { '-' } else { '+' };
                            let total = seconds.abs();
                            format!("{}{:02}", sign, total / 3600)
                        }
                    }
                    'X' | 'x' if count == 2 => {
                        if seconds == 0 && c == 'X' {
                            "Z".to_string()
                        } else {
                            let sign = if seconds < 0 { '-' } else { '+' };
                            let total = seconds.abs();
                            format!("{}{:02}{:02}", sign, total / 3600, (total % 3600) / 60)
                        }
                    }
                    _ => {
                        if seconds == 0 && c != 'x' && c != 'O' {
                            "Z".to_string()
                        } else {
                            format_offset(seconds)
                        }
                    }
                }
            } else {
                String::new()
            }
        }
        other => {
            let mut out = String::new();
            for _ in 0..count {
                out.push(other);
            }
            out
        }
    }
}

pub fn format_pattern(
    date: Option<&Date>,
    time: Option<&LocalTime>,
    zone: Option<&Zone>,
    pattern: &str,
) -> String {
    let naive_date = date.and_then(naive_date);
    let naive_time = time.and_then(naive_time);
    let mut out = String::new();
    for token in compile_pattern(pattern) {
        match token {
            PatternToken::Literal(text) => out.push_str(&text),
            PatternToken::Field(c, count) => out.push_str(&field_text(
                c,
                count,
                naive_date.as_ref(),
                naive_time.as_ref(),
                zone,
            )),
        }
    }
    out
}

fn read_digits(input: &[char], position: &mut usize, min: usize, max: usize) -> Option<String> {
    let start = *position;
    while *position < input.len() && *position - start < max && input[*position].is_ascii_digit() {
        *position += 1;
    }
    if *position - start < min {
        return None;
    }
    Some(input[start..*position].iter().collect())
}

fn read_letters(input: &[char], position: &mut usize) -> Option<String> {
    let start = *position;
    while *position < input.len() && input[*position].is_ascii_alphabetic() {
        *position += 1;
    }
    if *position == start {
        return None;
    }
    Some(input[start..*position].iter().collect())
}

fn skip_spaces(input: &[char], position: &mut usize) {
    while *position < input.len() && input[*position].is_whitespace() {
        *position += 1;
    }
}

fn parse_pattern_parts(
    input: &str,
    pattern: &str,
) -> Option<(Option<Date>, Option<LocalTime>, Option<Zone>)> {
    let chars: Vec<char> = input.chars().collect();
    let mut position = 0usize;
    let mut year = 1970i32;
    let mut month = 1u32;
    let mut day = 1u32;
    let mut hour = 0u32;
    let mut minute = 0u32;
    let mut second = 0u32;
    let mut nano = 0u32;
    let mut am_pm: Option<bool> = None;
    let mut zone = None;
    let mut has_date = false;
    let mut has_time = false;
    for token in compile_pattern(pattern) {
        match token {
            PatternToken::Literal(text) => {
                if text.trim().is_empty() {
                    skip_spaces(&chars, &mut position);
                } else {
                    for expected in text.chars() {
                        if chars.get(position) == Some(&expected) {
                            position += 1;
                        } else if expected.is_whitespace() {
                            skip_spaces(&chars, &mut position);
                        } else {
                            return None;
                        }
                    }
                }
            }
            PatternToken::Field(c, count) => match c {
                'G' => {
                    let text = read_letters(&chars, &mut position)?;
                    if !text.eq_ignore_ascii_case("AD") && !text.eq_ignore_ascii_case("CE") {
                        return None;
                    }
                }
                'u' | 'y' | 'Y' => {
                    let text = read_digits(&chars, &mut position, count.clamp(1, 2), count.max(4))?;
                    let mut value: i32 = text.parse().ok()?;
                    if text.len() == 2 {
                        value += 2000;
                    }
                    year = value;
                    has_date = true;
                }
                'M' | 'L' => {
                    if count >= 3 {
                        let text = read_letters(&chars, &mut position)?;
                        month = month_from_name(&text)?;
                    } else {
                        let text = read_digits(&chars, &mut position, 1, 2)?;
                        month = text.parse().ok()?;
                    }
                    has_date = true;
                }
                'd' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    day = text.parse().ok()?;
                    has_date = true;
                }
                'D' => {
                    let text = read_digits(&chars, &mut position, 1, 3)?;
                    let ordinal: u32 = text.parse().ok()?;
                    let date = NaiveDate::from_yo_opt(year, ordinal)?;
                    month = date.month();
                    day = date.day();
                    has_date = true;
                }
                'Q' | 'q' => {
                    let _ = read_digits(&chars, &mut position, 1, 1)?;
                }
                'E' | 'e' | 'c' => {
                    if count >= 3 {
                        let text = read_letters(&chars, &mut position)?;
                        weekday_from_name(&text)?;
                    } else {
                        let _ = read_digits(&chars, &mut position, 1, 2)?;
                    }
                }
                'w' | 'W' | 'F' => {
                    let _ = read_digits(&chars, &mut position, 1, 2)?;
                }
                'H' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    hour = text.parse().ok()?;
                    has_time = true;
                }
                'h' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    hour = text.parse().ok()?;
                    has_time = true;
                }
                'K' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    hour = text.parse().ok()?;
                    has_time = true;
                }
                'k' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    let value: u32 = text.parse().ok()?;
                    hour = if value == 24 { 0 } else { value };
                    has_time = true;
                }
                'm' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    minute = text.parse().ok()?;
                    has_time = true;
                }
                's' => {
                    let text = read_digits(&chars, &mut position, 1, 2)?;
                    second = text.parse().ok()?;
                    has_time = true;
                }
                'S' | 'n' => {
                    let max = if c == 'n' { 9 } else { count.max(1) };
                    let text = read_digits(&chars, &mut position, 1, max)?;
                    nano = parse_fraction(&text)?;
                    has_time = true;
                }
                'A' => {
                    let _ = read_digits(&chars, &mut position, 1, 12)?;
                }
                'N' => {
                    let _ = read_digits(&chars, &mut position, 1, 18)?;
                }
                'a' => {
                    let text = read_letters(&chars, &mut position)?;
                    am_pm = Some(text.eq_ignore_ascii_case("PM"));
                }
                'z' | 'Z' | 'X' | 'x' | 'O' | 'V' => {
                    skip_spaces(&chars, &mut position);
                    let start = position;
                    while position < chars.len()
                        && !chars[position].is_whitespace()
                        && !chars[position].is_ascii_alphabetic()
                        || (position == start && chars.get(position) == Some(&'Z'))
                        || (position == start && chars.get(position) == Some(&'z'))
                        || chars.get(position) == Some(&'/')
                    {
                        position += 1;
                    }
                    if position == start {
                        let text = read_letters(&chars, &mut position)?;
                        let _ = text;
                    } else {
                        let text: String = chars[start..position].iter().collect();
                        zone = Zone::parse(&text);
                    }
                }
                _ => {
                    let _ = read_digits(&chars, &mut position, 1, count.max(1));
                }
            },
        }
    }
    skip_spaces(&chars, &mut position);
    if position < chars.len() {
        return None;
    }
    if let Some(pm) = am_pm {
        if pm && hour < 12 {
            hour += 12;
        } else if !pm && hour == 12 {
            hour = 0;
        }
    }
    let date = if has_date {
        NaiveDate::from_ymd_opt(year, month, day)?;
        Some(Date { year, month, day })
    } else {
        None
    };
    let time = if has_time {
        chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano)?;
        Some(LocalTime {
            hour,
            minute,
            second,
            nano,
        })
    } else {
        None
    };
    Some((date, time, zone))
}

pub fn parse_date_pattern(text: &str, pattern: &str) -> Option<Date> {
    let (date, _, _) = parse_pattern_parts(text, pattern)?;
    date
}

pub fn parse_local_time_pattern(text: &str, pattern: &str) -> Option<LocalTime> {
    let (_, time, _) = parse_pattern_parts(text, pattern)?;
    time
}

pub fn parse_local_date_time_pattern(text: &str, pattern: &str) -> Option<LocalDateTime> {
    let (date, time, _) = parse_pattern_parts(text, pattern)?;
    Some(LocalDateTime {
        date: date?,
        time: time?,
    })
}

pub fn parse_date_time_pattern(text: &str, pattern: &str) -> Option<DateTime> {
    let (date, time, zone) = parse_pattern_parts(text, pattern)?;
    Some(DateTime {
        date: date?,
        time: time?,
        zone: zone.unwrap_or_else(Zone::utc),
    })
}

pub fn parse_time_pattern(text: &str, pattern: &str) -> Option<Time> {
    let (_, time, zone) = parse_pattern_parts(text, pattern)?;
    Some(Time {
        time: time?,
        zone: zone.unwrap_or_else(Zone::utc),
    })
}
