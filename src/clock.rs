use chrono::DateTime;
use chrono::Utc;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

pub struct FakeClock {
    pub now: DateTime<Utc>,
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}

impl FakeClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now: now }
    }
}
