use daggy::NodeIndex;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct TaskId(pub NodeIndex);
