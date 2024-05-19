use {
    chrono::Duration,
    serde_derive::{Deserialize, Serialize},
};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Default,
)]
pub struct DurationInSeconds(pub u32);

impl From<Duration> for DurationInSeconds {
    fn from(duration: Duration) -> Self {
        DurationInSeconds(duration.num_seconds() as u32)
    }
}
