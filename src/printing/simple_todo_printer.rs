use super::format_util::format_number;
use super::PrintableError;
use super::PrintableTask;
use super::PrintableWarning;
use super::TodoPrinter;
use ansi_term::Color;
use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Utc;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Write;

pub struct PrintingContext {
    /// The number of digits that task numbers may have, including a minus sign.
    pub max_index_digits: usize,
    /// The number of columns to render task descriptions in.
    pub width: usize,
    /// The current time.
    pub now: DateTime<Utc>,
}

pub struct SimpleTodoPrinter<Out: Write> {
    pub out: Out,
    pub context: PrintingContext,
}

struct PrintableTaskWithContext<'a> {
    context: &'a PrintingContext,
    task: &'a PrintableTask<'a>,
}
enum Urgency {
    Meh,
    Moderate,
    Urgent,
}

fn calculate_urgency(now: DateTime<Utc>, then: DateTime<Utc>) -> Urgency {
    if then - now < Duration::zero() {
        Urgency::Urgent
    } else if then - now < Duration::days(1) {
        Urgency::Moderate
    } else {
        Urgency::Meh
    }
}

fn calculate_progress(
    now: DateTime<Utc>,
    due: DateTime<Utc>,
    budget: Duration,
) -> i32 {
    let start = due - budget;
    let elapsed = now - start;
    let budget_spent: f64 =
        elapsed.num_seconds() as f64 / budget.num_seconds() as f64;

    (budget_spent * 100.0) as i32
}

#[cfg(test)]
#[test]
fn calculate_progress_test() {
    #![allow(clippy::zero_prefixed_literal)]
    use app::testing::ymdhms;
    assert_eq!(
        0,
        calculate_progress(
            ymdhms(2021, 04, 30, 10, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        50,
        calculate_progress(
            ymdhms(2021, 04, 30, 11, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        100,
        calculate_progress(
            ymdhms(2021, 04, 30, 12, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        -100,
        calculate_progress(
            ymdhms(2021, 04, 30, 08, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
    assert_eq!(
        200,
        calculate_progress(
            ymdhms(2021, 04, 30, 14, 00, 00),
            ymdhms(2021, 04, 30, 12, 00, 00),
            Duration::hours(2)
        )
    );
}

const ANSI_OFFSET: usize = 10;
const SELECTOR_OFFSET: usize = 6;
const LOG_DATE_OFFSET: usize = 11;

impl<'a> Display for PrintableTaskWithContext<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut start = if let Some(log_date) = &self.task.log_date {
            format!(
                "{} {} {:>width$} ",
                log_date,
                self.task.action,
                format_number(self.task.number, self.task.status),
                width = self.context.max_index_digits + ANSI_OFFSET
            )
        } else {
            format!(
                "{} {:>width$} ",
                self.task.action,
                format_number(self.task.number, self.task.status),
                width = self.context.max_index_digits + ANSI_OFFSET
            )
        };
        if let Some(start_date) = self.task.start_date {
            let snooze_duration = start_date - self.context.now;
            if snooze_duration > chrono::Duration::zero() {
                start.push_str(
                    &Color::Purple
                        .bold()
                        .paint(format!(
                            "Snoozed for {}",
                            ::time_format::format_duration_laconic(
                                snooze_duration
                            )
                        ))
                        .to_string(),
                );
                start.push(' ');
            }
        }
        if self.task.priority != 0 {
            let color = match self.task.priority.abs() {
                6..=i32::MAX => Color::Red,
                5 => Color::Yellow,
                4 => Color::Green,
                3 => Color::Cyan,
                2 => Color::Blue,
                1 => Color::Purple,
                _ => Color::Black,
            };
            let style = if self.task.priority >= 0 {
                color.bold()
            } else {
                color.bold().dimmed()
            };
            start.push_str(
                &style.paint(format!("P{}", self.task.priority)).to_string(),
            );
            start.push(' ');
        }
        if let Some(due_date) = self.task.due_date {
            let style = match calculate_urgency(self.context.now, due_date) {
                Urgency::Urgent => Color::Red.bold(),
                Urgency::Moderate => Color::Yellow.bold(),
                Urgency::Meh => Color::White.bold().dimmed(),
            };
            let desc = ::time_format::display_relative_time(
                self.context.now.with_timezone(&Local),
                due_date.with_timezone(&Local),
            );
            start.push_str(&style.paint(format!("Due {}", desc)).to_string());
            start.push(' ');
            if let Some(budget) = self.task.budget {
                let target_progress =
                    calculate_progress(self.context.now, due_date, budget);
                if (0..=100).contains(&target_progress) {
                    start.push_str(
                        &Color::White
                            .bold()
                            .paint("Target progress")
                            .to_string(),
                    );
                    start.push(' ');
                    let style = if target_progress < 50 {
                        Color::White.bold().dimmed()
                    } else if target_progress < 80 {
                        Color::Yellow.bold()
                    } else {
                        Color::Red.bold()
                    };
                    start.push_str(
                        &style
                            .paint(format!("{}%", target_progress))
                            .to_string(),
                    );
                    start.push(' ');
                }
            }
        }
        // If the task has deps, show a lock icon, followed by the number of
        // incomplete deps and the number of total deps, as a fraction. E.g.
        // if the task has 3 deps, 2 of which are incomplete, show "🔓 2/3".
        let (incomplete, total) = self.task.deps_stats;
        if total > 0 {
            start.push_str(
                &Color::Yellow
                    .paint(format!("🔒{}/{}", incomplete, total))
                    .to_string(),
            );
            start.push(' ');
        }
        // If the task has adeps, show an unlock icon, followed by the number of
        // unlockable adeps and the number of total adeps, as a fraction. E.g.
        // if the task would unlock two adeps when it is completed, out of three
        // total adeps, show "🔓2/3".
        //
        // If none of the adeps are unlockable, the first number is 0.
        //
        // If the task has no adeps, show nothing.
        let (unlockable, total) = self.task.adeps_stats;
        if total > 0 {
            start.push_str(
                &Color::White
                    .paint(format!("🔓{unlockable}/{total}"))
                    .to_string(),
            );
            start.push(' ');
        }
        write!(
            f,
            "{}",
            textwrap::fill(
                self.task.desc,
                textwrap::Options::new(self.context.width)
                    .initial_indent(&start)
                    .break_words(false)
                    .subsequent_indent(&" ".repeat(
                        self.context.max_index_digits
                            + SELECTOR_OFFSET
                            + if self.task.log_date.is_some() {
                                LOG_DATE_OFFSET
                            } else {
                                0
                            }
                    )),
            )
        )
    }
}

impl<Out: Write> TodoPrinter for SimpleTodoPrinter<Out> {
    fn print_task(&mut self, task: &PrintableTask) {
        writeln!(
            self.out,
            "{}",
            PrintableTaskWithContext {
                context: &self.context,
                task,
            }
        ).unwrap_or_default();
    }
    fn print_warning(&mut self, warning: &PrintableWarning) {
        writeln!(self.out, "{}", warning).unwrap_or_default();
    }
    fn print_error(&mut self, error: &PrintableError) {
        writeln!(self.out, "{}", error).unwrap_or_default();
    }
}
