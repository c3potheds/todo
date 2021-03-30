extern crate humantime;

use chrono::DateTime;
use chrono::Datelike;
use chrono::TimeZone;

#[derive(Debug)]
pub struct ParseTimeError;

enum Midi {
    Am,
    Pm,
}

enum ParseTimeOfDayState {
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
                hour: s[start..end].parse::<u8>().unwrap(),
                minute_so_far: (end + 1, end + 1),
            })
        }
        (ParsingNumber { number_so_far }, 'a') => {
            let (start, end) = number_so_far;
            Ok(ExpectingAm {
                hour: s[start..end].parse::<u8>().unwrap(),
                minute: 00,
            })
        }
        (ParsingNumber { number_so_far }, 'p') => {
            let (start, end) = number_so_far;
            Ok(ExpectingPm {
                hour: s[start..end].parse::<u8>().unwrap(),
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
                hour: hour,
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
                hour: hour,
                minute: s[start..end].parse::<u8>().unwrap(),
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
                hour: hour,
                minute: s[start..end].parse::<u8>().unwrap(),
            })
        }
        (ExpectingAm { hour, minute }, 'm') => Ok(FullInfo {
            hour: hour,
            minute: minute,
            midi: Midi::Am,
        }),
        (ExpectingPm { hour, minute }, 'm') => Ok(FullInfo {
            hour: hour,
            minute: minute,
            midi: Midi::Pm,
        }),
        _ => Err(ParseTimeError),
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
                let hour = (hour
                    + match midi {
                        Midi::Am => 0,
                        Midi::Pm => 12,
                    }) as u32;
                let mut target = tz
                    .ymd(now.year(), now.month(), now.day())
                    .and_hms(hour, minute as u32, 00);
                if target < now {
                    target = target.with_day(target.day() + 1).unwrap();
                }
                Ok(target)
            }
            _ => Err(ParseTimeError),
        })
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

fn end_of_month<Tz: TimeZone>(datetime: DateTime<Tz>) -> DateTime<Tz> {
    // Increment the datetime by a day until the month changes.
    let this_month = datetime.month();
    let mut forward = datetime.clone();
    loop {
        let next = forward.clone() + chrono::Duration::days(1);
        if next.month() != this_month {
            return end_of_day(forward);
        }
        forward = next;
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
) -> Result<DateTime<Tz>, ParseTimeError> {
    use std::str::FromStr;
    chrono::Weekday::from_str(s)
        .map(|weekday| {
            let mut fast_forwarded = now + chrono::Duration::days(1);
            while fast_forwarded.weekday() != weekday {
                fast_forwarded = fast_forwarded + chrono::Duration::days(1);
            }
            end_of_day(fast_forwarded)
        })
        .map_err(|_| ParseTimeError)
}

pub fn parse_time<Tz: TimeZone>(
    tz: Tz,
    now: DateTime<Tz>,
    s: &str,
) -> Result<DateTime<Tz>, ParseTimeError> {
    humantime::parse_duration(s)
        .map(|duration: std::time::Duration| {
            let mut datetime = now.clone()
                + chrono::Duration::milliseconds(duration.as_millis() as i64);
            if chrono::Duration::days(1).to_std().unwrap() <= duration {
                datetime = end_of_day(datetime);
            }
            datetime
        })
        .or_else(|_| parse_day_of_week(now.clone(), s))
        .or_else(|_| {
            if s == "today" {
                Ok(end_of_day(now.clone()))
            } else {
                Err(ParseTimeError)
            }
        })
        .or_else(|_| {
            if s == "tomorrow" {
                Ok(end_of_day(now.clone() + chrono::Duration::days(1)))
            } else {
                Err(ParseTimeError)
            }
        })
        .or_else(|_| {
            let mut chunks = s.split_whitespace();
            match chunks.next() {
                Some("last") => match chunks.next() {
                    Some(dow) => parse_day_of_week(
                        now.clone() - chrono::Duration::days(1),
                        dow,
                    )
                    .map(|datetime| datetime - chrono::Duration::days(7)),
                    _ => Err(ParseTimeError),
                },
                Some(chunk) => chunk
                    .parse::<chrono::Month>()
                    .map_err(|_| ParseTimeError)
                    .and_then(|month| match chunks.next() {
                        Some(chunk) => chunk
                            .parse::<u32>()
                            .map_err(|_| ParseTimeError)
                            .map(|day| {
                                end_of_month_after(now.clone(), month)
                                    .with_day(day)
                                    .unwrap()
                            }),
                        None => Ok(end_of_month_after(now.clone(), month)),
                    }),
                _ => Err(ParseTimeError),
            }
        })
        .or_else(|_| parse_time_of_day(tz, now.clone(), s))
}

// The humantime::format_duration() function will format durations like "5m 32s"
// to however much precision is representable. For "laconic" representation of
// duration, presented to the user, we don't need second-level precision for
// durations in the order of minutes, or minute-level precision for durations
// in the order of hourse, etc, so we strip off all but the first "word" in the
// formatted time.
fn format_duration_laconic(duration: chrono::Duration) -> String {
    let formatted =
        humantime::format_duration(duration.to_std().unwrap().into());
    match format!("{}", formatted).split(" ").next() {
        Some(chunk) => chunk.to_string(),
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
