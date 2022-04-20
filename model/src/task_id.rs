use {
    daggy::NodeIndex,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(pub NodeIndex);
