use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub enum TaskStatus {
    Complete,
    Incomplete,
    Blocked,
}
