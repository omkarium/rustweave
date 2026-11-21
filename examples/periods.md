# Periods functions in one script

Periods are built from components, measured between dates, added to and subtracted from temporal values, and read back through selectors.

This script uses all 9 functions listed for `dw::core::Periods`.

Import `dw::core::Periods` to use the module functions.

## Script

```dataweave
%dw 2.0
import * from dw::core::Periods
output application/json
---
{
  between: between(|2014-11-20|, |2014-11-18|),
  since: between(|2015-10-21|, |2014-09-19|),
  days: days(3),
  hours: hours(3.75),
  minutes: minutes(3.75),
  seconds: seconds(3.75),
  months: months(3),
  years: years(3),
  duration: duration({days: 3, hours: 7, minutes: 42}),
  period: period({years: 3, months: 7, days: 42}),
  addDays: |2021-02-14T11:30:15Z| + days(2),
  subtractHours: |2021-02-14T11:30:15Z| - hours(3),
  addMonths: |2021-01-31| + months(1),
  addYears: |2020-02-29| + years(1),
  durationHours: duration({days: 3, hours: 7, minutes: 42}).hours,
  periodMonths: period({years: 3, months: 7, days: 42}).months
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/periods.dwl
```

## Expected output

```json
{
  "between": "P2D",
  "since": "P1Y1M2D",
  "days": "P3D",
  "hours": "PT3H45M",
  "minutes": "PT3M45S",
  "seconds": "PT3.75S",
  "months": "P3M",
  "years": "P3Y",
  "duration": "PT79H42M",
  "period": "P3Y7M42D",
  "addDays": "2021-02-16T11:30:15Z",
  "subtractHours": "2021-02-14T08:30:15Z",
  "addMonths": "2021-02-28",
  "addYears": "2021-02-28",
  "durationHours": 7,
  "periodMonths": 7
}
```

## Functions shown

`between`, `days`, `duration`, `hours`, `minutes`, `months`, `period`, `seconds`, `years`
