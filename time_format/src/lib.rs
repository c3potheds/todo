use chrono::DateTime;
use chrono::Datelike;
use chrono::TimeZone;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseTimeError {
    LocalError(chrono::format::ParseError),
    InvalidHour(std::num::ParseIntError),
    InvalidMinute(std::num::ParseIntError),
    InvalidSecond(std::num::ParseIntError),
    UnexpectedChar(char),
    InvalidTimeOfDay(u8, Midi),
    IncompleteTimeOfDay(ParseTimeOfDayState),
    InvalidWeekday(chrono::ParseWeekdayError),
    InvalidMonth(chrono::ParseMonthError),
    DayOfMonthIsNotNumber(std::num::ParseIntError),
    InvalidDayOfMonth(chrono::Month, u32),
    InvalidYear(std::num::ParseIntError),
    Misc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Midi {
    Am,
    Pm,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseTimeOfDayState {
    ParsingNumber {
        number_so_far: (usize, usize),
    },
    ParsingTimeOClock {
        hour: u8,
        minute_so_far: (usize, usize),
    },
    ExpectingAm {
        hour: u8,
        minute: u8,
    },
    ExpectingPm {
        hour: u8,
        minute: u8,
    },
    FullInfo {
        hour: u8,
        minute: u8,
        midi: Midi,
    },
}

fn parse_time_of_day_step(
    s: &str,
    state: ParseTimeOfDayState,
    c: char,
) -> Result<ParseTimeOfDayState, ParseTimeError> {
    use self::ParseTimeOfDayState::*;
    match (state, c) {
        (ParsingNumber { number_so_far }, '0'..='9') => {
            let (start, end) = number_so_far;
            Ok(ParsingNumber {
                number_so_far: (start, end + 1),
            })
        }
        (ParsingNumber { number_so_far }, ':') => {
            let (start, end) = number_so_far;
            Ok(ParsingTimeOClock {
                hour: s[start..end]
                    .parse::<u8>()
                    .map_err(ParseTimeError::InvalidHour)?,
                minute_so_far: (end + 1, end + 1),
            })
        }
        (ParsingNumber { number_so_far }, 'a') => {
            let (start, end) = number_so_far;
            Ok(ExpectingAm {
                hour: s[start..end]
                    .parse::<u8>()
                    .map_err(ParseTimeError::InvalidHour)?,
                minute: 00,
            })
        }
        (ParsingNumber { number_so_far }, 'p') => {
            let (start, end) = number_so_far;
            Ok(ExpectingPm {
                hour: s[start..end]
                    .parse::<u8>()
                    .map_err(ParseTimeError::InvalidHour)?,
                minute: 00,
            })
        }
        (
            ParsingTimeOClock {
                hour,
                minute_so_far,
            },
            '0'..='9',
        ) => {
            let (start, end) = minute_so_far;
            Ok(ParsingTimeOClock {
                hour,
                minute_so_far: (start, end + 1),
            })
        }
        (
            ParsingTimeOClock {
                hour,
                minute_so_far,
            },
            'a',
        ) => {
            let (start, end) = minute_so_far;
            Ok(ExpectingAm {
                hour,
                minute: s[start..end]
                    .parse::<u8>()
                    .map_err(ParseTimeError::InvalidMinute)?,
            })
        }
        (
            ParsingTimeOClock {
                hour,
                minute_so_far,
            },
            'p',
        ) => {
            let (start, end) = minute_so_far;
            Ok(ExpectingPm {
                hour,
                minute: s[start..end]
                    .parse::<u8>()
                    .map_err(ParseTimeError::InvalidMinute)?,
            })
        }
        (ExpectingAm { hour, minute }, 'm') => Ok(FullInfo {
            hour,
            minute,
            midi: Midi::Am,
        }),
        (ExpectingPm { hour, minute }, 'm') => Ok(FullInfo {
            hour,
            minute,
            midi: Midi::Pm,
        }),
        (_, c) => Err(ParseTimeError::UnexpectedChar(c)),
    }
}

fn parse_time_of_day<Tz: TimeZone>(
    tz: Tz,
    now: DateTime<Tz>,
    s: &str,
) -> Result<DateTime<Tz>, ParseTimeError> {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .try_fold(
            ParseTimeOfDayState::ParsingNumber {
                number_so_far: (0, 0),
            },
            |state, c| parse_time_of_day_step(s, state, c),
        )
        .and_then(|state| match state {
            ParseTimeOfDayState::FullInfo { hour, minute, midi } => {
                let hour = match (hour, midi) {
                    (0..=11, Midi::Am) => hour,
                    (0..=11, Midi::Pm) => hour + 12,
                    (12, Midi::Am) => 0,
                    (12, Midi::Pm) => 12,
                    _ => {
                        return Err(ParseTimeError::InvalidTimeOfDay(
                            hour, midi,
                        ))
                    }
                };
                let mut target = tz
                    .with_ymd_and_hms(
                        now.year(),
                        now.month(),
                        now.day(),
                        hour as u32,
                        minute as u32,
                        00,
                    )
                    .unwrap();
                if target < now {
                    target += chrono::Duration::days(1);
                }
                Ok(target)
            }
            state => Err(ParseTimeError::IncompleteTimeOfDay(state)),
        })
}

fn start_of_day<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    use chrono::Timelike;
    datetime
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

fn end_of_day<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    use chrono::Timelike;
    datetime
        .with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
}

fn start_of_month<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    start_of_day(datetime.with_day(1).unwrap())
}

fn end_of_month<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    // Increment the datetime by a day until the month changes.
    let this_month = datetime.month();
    let mut forward = datetime;
    loop {
        let next = forward.clone() + chrono::Duration::days(1);
        if next.month() != this_month {
            return end_of_day(forward);
        }
        forward = next;
    }
}

fn start_of_month_after<Tz: TimeZone>(
    datetime: DateTime<Tz>,
    month: chrono::Month,
) -> DateTime<Tz> {
    if datetime.month() == month.number_from_month() {
        start_of_month(datetime)
    } else {
        start_of_month_after(datetime + chrono::Duration::days(28), month)
    }
}

fn end_of_month_after<Tz: TimeZone>(
    datetime: DateTime<Tz>,
    month: chrono::Month,
) -> DateTime<Tz> {
    if datetime.month() == month.number_from_month() {
        end_of_month(datetime)
    } else {
        end_of_month_after(datetime + chrono::Duration::days(28), month)
    }
}

fn parse_day_of_week<Tz: TimeZone>(
    now: DateTime<Tz>,
    s: &str,
    snap: Snap,
) -> Result<DateTime<Tz>, ParseTimeError> {
    use std::str::FromStr;
    let weekday =
        chrono::Weekday::from_str(s).map_err(ParseTimeError::InvalidWeekday)?;
    let mut fast_forwarded = now + chrono::Duration::days(1);
    while fast_forwarded.weekday() != weekday {
        fast_forwarded += chrono::Duration::days(1);
    }
    Ok(match snap {
        Snap::ToStart => start_of_day(fast_forwarded),
        Snap::ToEnd => end_of_day(fast_forwarded),
    })
}

#[derive(Clone, Copy)]
pub enum Snap {
    ToStart,
    ToEnd,
}

fn parse_month_day<'a, Tz: TimeZone>(
    now: DateTime<Tz>,
    chunk: &str,
    chunks: &mut impl Iterator<Item = &'a str>,
    snap: Snap,
) -> Result<DateTime<Tz>, ParseTimeError> {
    chunk
        .parse::<chrono::Month>()
        .map_err(ParseTimeError::InvalidMonth)
        .and_then(|month| match chunks.next() {
            Some(chunk) => chunk
                .parse::<u32>()
                .map_err(ParseTimeError::DayOfMonthIsNotNumber)
                .and_then(|day| {
                    let datetime = end_of_month_after(now.clone(), month)
                        .with_day(day)
                        .ok_or(ParseTimeError::InvalidDayOfMonth(month, day))?;
                    Ok(match snap {
                        Snap::ToStart => start_of_day(datetime),
                        Snap::ToEnd => datetime,
                    })
                }),
            None => Ok(match snap {
                Snap::ToStart => start_of_month_after(now.clone(), month),
                Snap::ToEnd => end_of_month_after(now.clone(), month),
            }),
        })
}

fn parse_year_month_day<'a, Tz: TimeZone>(
    tz: Tz,
    chunk: &str,
    chunks: &mut impl Iterator<Item = &'a str>,
    snap: Snap,
) -> Result<DateTime<Tz>, ParseTimeError> {
    #![allow(clippy::zero_prefixed_literal)]
    // Year must be formatted as YYYY.
    let year = chunk.parse::<i32>().map_err(ParseTimeError::InvalidYear)?;
    match chunks.next() {
        Some(chunk) => parse_month_day(
            tz.with_ymd_and_hms(year, 01, 01, 00, 00, 00).unwrap(),
            chunk,
            chunks,
            snap,
        )
        .map(|datetime| datetime.with_year(year).unwrap()),
        None => Ok(match snap {
            Snap::ToStart => tz.with_ymd_and_hms(year, 01, 01, 00, 00, 00),
            Snap::ToEnd => tz.with_ymd_and_hms(year, 12, 31, 23, 59, 59),
        }
        .unwrap()),
    }
}

pub fn parse_time<Tz: TimeZone>(
    tz: Tz,
    now: DateTime<Tz>,
    s: &str,
    snap: Snap,
) -> Result<DateTime<Tz>, ParseTimeError> {
    humantime::parse_duration(s)
        .map(|duration: std::time::Duration| {
            let mut datetime = now.clone()
                + chrono::Duration::milliseconds(duration.as_millis() as i64);
            if chrono::Duration::days(1).to_std().unwrap() <= duration {
                datetime = match snap {
                    Snap::ToStart => start_of_day(datetime),
                    Snap::ToEnd => end_of_day(datetime),
                }
            };
            datetime
        })
        .or_else(|_| parse_day_of_week(now.clone(), s, snap))
        .or_else(|_| {
            if s == "today" {
                match snap {
                    Snap::ToStart => Ok(start_of_day(now.clone())),
                    Snap::ToEnd => Ok(end_of_day(now.clone())),
                }
            } else {
                Err(ParseTimeError::Misc)
            }
        })
        .or_else(|_| {
            if s == "tomorrow" {
                match snap {
                    Snap::ToStart => Ok(start_of_day(
                        now.clone() + chrono::Duration::days(1),
                    )),
                    Snap::ToEnd => {
                        Ok(end_of_day(now.clone() + chrono::Duration::days(1)))
                    }
                }
            } else {
                Err(ParseTimeError::Misc)
            }
        })
        .or_else(|_| {
            let mut chunks = s.split_whitespace();
            match chunks.next() {
                Some("last") => match chunks.next() {
                    Some(dow) => parse_day_of_week(
                        now.clone() - chrono::Duration::days(1),
                        dow,
                        snap,
                    )
                    .map(|datetime| datetime - chrono::Duration::days(7)),
                    _ => Err(ParseTimeError::Misc),
                },
                Some(chunk) => {
                    parse_month_day(now.clone(), chunk, &mut chunks, snap)
                        .or_else(|_| {
                            parse_year_month_day(
                                tz.clone(),
                                chunk,
                                &mut chunks,
                                snap,
                            )
                        })
                }
                _ => Err(ParseTimeError::Misc),
            }
        })
        .or_else(|_| parse_time_of_day(tz, now.clone(), s))
}

// The humantime::format_duration() function will format durations like "5m 32s"
// to however much precision is representable. For "laconic" representation of
// duration, presented to the user, we don't need second-level precision for
// durations in the order of minutes, or minute-level precision for durations
// in the order of hours, etc, so we strip off all but the first "word" in the
// formatted time.
pub fn format_duration_laconic(duration: chrono::Duration) -> String {
    let formatted = humantime::format_duration(duration.to_std().unwrap());
    match format!("{}", formatted).split(' ').next() {
        Some(chunk) => {
            let len = chunk.chars().take_while(|c| c.is_ascii_digit()).count();
            let n = &chunk[0..len];
            let unit = match (n.parse::<i32>().unwrap(), &chunk[len..]) {
                (1, "s") => "second",
                (_, "s") => "seconds",
                (1, "m") => "minute",
                (_, "m") => "minutes",
                (1, "h") => "hour",
                (_, "h") => "hours",
                _ => &chunk[len..],
            };
            [n, unit].join(" ")
        }
        None => panic!("Formatted duration is empty string: {}", formatted),
    }
}

pub fn display_relative_time<Tz: TimeZone>(
    now: DateTime<Tz>,
    then: DateTime<Tz>,
) -> String {
    let duration = then - now;
    if duration < chrono::Duration::zero() {
        format!("{} ago", format_duration_laconic(-duration))
    } else {
        format!("in {}", format_duration_laconic(duration))
    }
}

#[cfg(test)]
mod test;
