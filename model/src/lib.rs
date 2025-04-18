mod duration;
mod layering;
mod task;
mod task_id;
mod task_set;
mod task_status;
mod todo_list;

pub use self::duration::*;
pub use self::task::*;
pub use self::task_id::*;
pub use self::task_set::*;
pub use self::task_status::*;
pub use self::todo_list::*;

use chrono::{DateTime, NaiveTime, Utc, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusPredicate {
    Weekdays(HashSet<Weekday>),
    TimeOfDayRange { start: NaiveTime, end: NaiveTime },
    DateTimeRange { start: DateTime<Utc>, end: DateTime<Utc> },
}

pub fn check_predicate(predicate: &FocusPredicate, now: DateTime<Utc>) -> bool {
    use chrono::Timelike; // Import Timelike trait for now.time()
    match predicate {
        FocusPredicate::Weekdays(days) => {
            days.contains(&now.weekday())
        }
        FocusPredicate::TimeOfDayRange { start, end } => {
            let current_time = now.time();
            if start <= end {
                // Standard range (e.g., 9am - 5pm)
                current_time >= *start && current_time < *end
            } else {
                // Wrap-around range (e.g., 10pm - 2am)
                current_time >= *start || current_time < *end
            }
        }
        FocusPredicate::DateTimeRange { start, end } => {
            now >= *start && now < *end
        }
    }
}


#[cfg(test)]
mod task_test;

#[cfg(test)]
mod focus_test; // Register the new test module

#[cfg(test)]
mod todo_list_test;
