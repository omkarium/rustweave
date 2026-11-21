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
