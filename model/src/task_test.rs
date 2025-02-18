#![allow(clippy::zero_prefixed_literal)]

use chrono::TimeZone;
use chrono::Utc;

use super::*;

#[test]
fn deserialize_task_with_missing_creation_time() {
    let task = serde_json::from_str::<Task>("{\"desc\":\"hi\"}")
        .ok()
        .unwrap();
    assert_eq!(task.desc, "hi");
    assert_ne!(
        task.creation_time,
        Utc.with_ymd_and_hms(1970, 01, 01, 00, 00, 00).unwrap()
    );
    assert_eq!(task.completion_time, None);
    assert_eq!(task.priority, 0);
    assert_eq!(task.implicit_priority, 0);
    assert_eq!(task.due_date, None);
    assert_eq!(task.implicit_due_date, None);
    assert_eq!(task.budget, DurationInSeconds::default());
    assert!(!task.tag);
    assert_eq!(task.implicit_tags, vec![]);
}
