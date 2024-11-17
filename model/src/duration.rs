use chrono::Duration;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Default,
)]
pub struct DurationInSeconds(pub u32);

impl From<Duration> for DurationInSeconds {
    fn from(duration: Duration) -> Self {
        DurationInSeconds(duration.num_seconds() as u32)
    }
}
