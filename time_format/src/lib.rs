use chrono::DateTime;
use chrono::Datelike;
use chrono::{DateTime, Datelike, NaiveTime, TimeZone, Weekday};
use std::{collections::HashSet, str::FromStr};

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

#[derive(Debug, PartialEq, Eq)]
pub enum ParseFocusError {
    UnknownPredicateType(String),
    InvalidWeekdaySequence(String),
    InvalidTimeRangeFormat(String),
    ChronoParseError(chrono::format::ParseError),
    InvalidTime(String),
    MissingPart(String),
    ChronoParseWeekdayError(chrono::ParseWeekdayError),
    // Add more specific errors as needed
}

impl From<chrono::format::ParseError> for ParseFocusError {
    fn from(e: chrono::format::ParseError) -> Self {
        ParseFocusError::ChronoParseError(e)
    }
}

impl From<chrono::ParseWeekdayError> for ParseFocusError {
    fn from(e: chrono::ParseWeekdayError) -> Self {
        ParseFocusError::ChronoParseWeekdayError(e)
    }
}

// Helper to parse time strings like "14:30", "2pm", "09:15am" into NaiveTime
fn parse_naive_time(time_str: &str) -> Result<NaiveTime, ParseFocusError> {
    let time_str = time_str.trim();
    // Try parsing HH:MM (24-hour) first
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        return Ok(time);
    }
    // Try parsing HH (e.g., "14")
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H") {
         return Ok(time);
    }
     // Try parsing HHam/pm (e.g., "2pm")
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%I%p") {
        return Ok(time);
    }
    // Try parsing HH:MMam/pm (e.g., "02:30pm")
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%I:%M%p") {
        return Ok(time);
    }
     // Try parsing H am/pm (e.g., "2 pm") - Note: chrono doesn't directly support space, handle manually
     let lower = time_str.to_lowercase();
     if let Some(i) = lower.find(" am") {
         if let Ok(time) = NaiveTime::parse_from_str(&format!("{}am", &lower[..i].trim()), "%I%p") {
              return Ok(time);
         }
     }
     if let Some(i) = lower.find(" pm") {
         if let Ok(time) = NaiveTime::parse_from_str(&format!("{}pm", &lower[..i].trim()), "%I%p") {
             return Ok(time);
         }
     }


    Err(ParseFocusError::InvalidTime(time_str.to_string()))
}


pub fn parse_focus_predicate(
    s: &str,
) -> Result<model::FocusPredicate, ParseFocusError> {
    let lower = s.to_lowercase();
    match lower.as_str() {
        "weekdays" => Ok(model::FocusPredicate::Weekdays(
            vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
            ]
            .into_iter()
            .collect(),
        )),
        "weekends" => Ok(model::FocusPredicate::Weekdays(
            vec![Weekday::Sat, Weekday::Sun].into_iter().collect(),
        )),
        // Try parsing sequences of weekday abbreviations
        _ => {
            let mut weekdays = HashSet::new();
            let mut i = 0;
            while i < lower.len() {
                // Try matching known abbreviations (longest first)
                let remaining = &lower[i..];
                let mut found_match = false;
                for (abbr, day, len) in [
                    ("mon", Weekday::Mon, 3), ("monday", Weekday::Mon, 6),
                    ("tue", Weekday::Tue, 3), ("tuesday", Weekday::Tue, 7),
                    ("wed", Weekday::Wed, 3), ("wednesday", Weekday::Wed, 9),
                    ("thu", Weekday::Thu, 3), ("thursday", Weekday::Thu, 8),
                    ("fri", Weekday::Fri, 3), ("friday", Weekday::Fri, 6),
                    ("sat", Weekday::Sat, 3), ("saturday", Weekday::Sat, 8),
                    ("sun", Weekday::Sun, 3), ("sunday", Weekday::Sun, 6),
                    // Special cases
                    ("t", Weekday::Tue, 1), // Ambiguous 't' -> Tue is common
                    ("th", Weekday::Thu, 2),
                    ("w", Weekday::Wed, 1),
                    ("m", Weekday::Mon, 1),
                    ("f", Weekday::Fri, 1),
                    ("s", Weekday::Sat, 1), // Ambiguous 's' -> Sat is common
                 ] {
                    if remaining.starts_with(abbr) {
                        weekdays.insert(day);
                        i += len;
                        found_match = true;
                        break; // Found the longest match starting at i
                    }
                }
                if !found_match {
                    // If no abbreviation matched at this position, it's either the end
                    // or an invalid sequence, or it might be a time range.
                    // For now, assume invalid if weekdays set is not empty, else try time parsing.
                    if !weekdays.is_empty() {
                         return Err(ParseFocusError::InvalidWeekdaySequence(format!(
                            "Unexpected character sequence starting at index {} in '{}'",
                            i, s
                        )));
                    } else {
                         // Break here and try time parsing later
                         break;
                    }
                }
            }

            if !weekdays.is_empty() {
                // Successfully parsed at least one weekday and reached the end of the string
                 Ok(model::FocusPredicate::Weekdays(weekdays))
            } else {
                // If no weekdays were parsed, try parsing as a time range
                let lower = s.trim().to_lowercase(); // Use trimmed lowercase version
                if let Some(time_str) = lower.strip_prefix("after ") {
                     let start = parse_naive_time(time_str)?;
                     let end = NaiveTime::from_hms_opt(23, 59, 59).unwrap(); // End of day
                     Ok(model::FocusPredicate::TimeOfDayRange{ start, end })
                } else if let Some(time_str) = lower.strip_prefix("before ") {
                     let start = NaiveTime::from_hms_opt(0, 0, 0).unwrap(); // Start of day
                     let end = parse_naive_time(time_str)?;
                     Ok(model::FocusPredicate::TimeOfDayRange{ start, end })
                } else if let Some(separator_index) = lower.find('-') {
                    let start_str = &lower[..separator_index];
                    let end_str = &lower[separator_index + 1..];
                    let start = parse_naive_time(start_str)?;
                    let end = parse_naive_time(end_str)?;
                     // Basic validation: end time should be after start time?
                     // For now, allow wrapping around midnight (e.g., 10pm-2am)
                    Ok(model::FocusPredicate::TimeOfDayRange { start, end })
                } else {
                     // If it's not weekdays and not a recognizable time range format
                    Err(ParseFocusError::UnknownPredicateType(s.to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod test;
