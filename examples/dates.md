# Dates functions in one script

Fixed date, time, date-time, and local values are constructed and moved to the start of their day, hour, month, week, and year. The clock-relative helpers are compared with each other so the output stays stable.

This script uses all 13 functions listed for `dw::core::Dates`.

Import `dw::core::Dates` to use the module functions. `daysBetween` comes from Core.

## Script

```dataweave
%dw 2.0
import * from dw::core::Dates
output application/json
---
{
  date: date({year: 2021, month: 2, day: 14}),
  dateTime: dateTime({year: 2021, month: 2, day: 14, hour: 11, minutes: 30, seconds: 15, timeZone: |-05:00|}),
  localDateTime: localDateTime({year: 2021, month: 2, day: 14, hour: 11, minutes: 30, seconds: 15}),
  localTime: localTime({hour: 11, minutes: 30, seconds: 15}),
  time: time({hour: 11, minutes: 30, seconds: 15, timeZone: |-05:00|}),
  atBeginningOfDay: atBeginningOfDay(|2021-02-14T11:30:15-05:00|),
  atBeginningOfHour: atBeginningOfHour(|11:30:15.250|),
  atBeginningOfMonth: atBeginningOfMonth(|2021-02-14|),
  atBeginningOfWeek: atBeginningOfWeek(|2021-02-14|),
  atBeginningOfYear: atBeginningOfYear(|2021-02-14|),
  today: [typeOf(today()), daysBetween(today(), tomorrow())],
  tomorrow: [typeOf(tomorrow()), daysBetween(today(), tomorrow())],
  yesterday: [typeOf(yesterday()), daysBetween(yesterday(), today())]
}
```

## Run

```bash
./target/debug/rustweave -f examples/scripts/dates.dwl
```

## Expected output

```json
{
  "date": "2021-02-14",
  "dateTime": "2021-02-14T11:30:15-05:00",
  "localDateTime": "2021-02-14T11:30:15",
  "localTime": "11:30:15",
  "time": "11:30:15-05:00",
  "atBeginningOfDay": "2021-02-14T00:00:00-05:00",
  "atBeginningOfHour": "11:00:00",
  "atBeginningOfMonth": "2021-02-01",
  "atBeginningOfWeek": "2021-02-14",
  "atBeginningOfYear": "2021-01-01",
  "today": [
    "Date",
    1
  ],
  "tomorrow": [
    "Date",
    1
  ],
  "yesterday": [
    "Date",
    1
  ]
}
```

## Functions shown

`atBeginningOfDay`, `atBeginningOfHour`, `atBeginningOfMonth`, `atBeginningOfWeek`, `atBeginningOfYear`, `date`, `dateTime`, `localDateTime`, `localTime`, `time`, `today`, `tomorrow`, `yesterday`
