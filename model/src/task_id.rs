use {
    daggy::NodeIndex,
    serde_derive::{Deserialize, Serialize},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(pub NodeIndex);
