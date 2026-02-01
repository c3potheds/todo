use daggy::NodeIndex;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(pub NodeIndex);

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.index())
    }
}
