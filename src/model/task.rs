use chrono::DateTime;
use chrono::Utc;
use model::DurationInSeconds;
use std::borrow::Cow;

fn default_creation_time() -> DateTime<Utc> {
    Utc::now()
}

// NOTE: all new fields need to be Options or be marked #[serde(default)] to
// allow backwards compatibility.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Task<'ser> {
    #[serde(borrow)]
    pub desc: Cow<'ser, str>,
    #[serde(default = "default_creation_time")]
    pub creation_time: DateTime<Utc>,
    #[serde(default)]
    pub completion_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub implicit_priority: i32,
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub implicit_due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub budget: DurationInSeconds,
    #[serde(default = "default_creation_time")]
    pub start_date: DateTime<Utc>,
}

pub struct NewOptions<'ser> {
    pub desc: Cow<'ser, str>,
    pub now: DateTime<Utc>,
    pub priority: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub budget: DurationInSeconds,
    pub start_date: DateTime<Utc>,
}

impl<'ser> NewOptions<'ser> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            desc: Cow::Borrowed(""),
            now,
            priority: 0,
            due_date: None,
            budget: DurationInSeconds::default(),
            start_date: now,
        }
    }

    pub fn desc<S: Into<Cow<'ser, str>>>(mut self, desc: S) -> Self {
        self.desc = desc.into();
        self
    }

    pub fn creation_time(mut self, now: DateTime<Utc>) -> Self {
        self.now = now;
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn budget<D: Into<DurationInSeconds>>(mut self, budget: D) -> Self {
        self.budget = budget.into();
        self
    }

    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = start_date;
        self
    }
}

impl<'ser, S: Into<Cow<'ser, str>>> From<S> for NewOptions<'ser> {
    fn from(desc: S) -> Self {
        let now = Utc::now();
        Self {
            desc: desc.into(),
            now,
            priority: 0,
            due_date: None,
            budget: DurationInSeconds::default(),
            start_date: now,
        }
    }
}

impl<'ser> Task<'ser> {
    pub fn new<Options: Into<NewOptions<'ser>>>(
        options: Options,
    ) -> Task<'ser> {
        let options = options.into();
        Task {
            desc: options.desc,
            creation_time: options.now,
            completion_time: None,
            priority: options.priority,
            implicit_priority: options.priority,
            due_date: options.due_date,
            implicit_due_date: options.due_date,
            budget: options.budget,
            start_date: options.start_date,
        }
    }
}
