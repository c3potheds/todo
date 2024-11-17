use daggy::NodeIndex;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(pub NodeIndex);
