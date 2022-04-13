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

fn get_initial_indent(
    task: &PrintableTask,
    context: &PrintingContext,
) -> String {
    if let Some(log_date) = &task.log_date {
        format!(
            "{} {} {:>width$} ",
            log_date,
            task.action,
            format_number(task.number, task.status),
            width = context.max_index_digits + ANSI_OFFSET
        )
    } else {
        format!(
            "{} {:>width$} ",
            task.action,
            format_number(task.number, task.status),
            width = context.max_index_digits + ANSI_OFFSET
        )
    }
}

fn fmt_snooze_date(snooze_duration: Duration, out: &mut String) {
    if snooze_duration > chrono::Duration::zero() {
        out.push_str(
            &Color::Purple
                .bold()
                .paint(format!(
                    "Snoozed for {}",
                    ::time_format::format_duration_laconic(snooze_duration)
                ))
                .to_string(),
        );
        out.push(' ');
    }
}

fn fmt_priority(priority: i32, out: &mut String) {
    let color = match priority.abs() {
        6..=i32::MAX => Color::Red,
        5 => Color::Yellow,
        4 => Color::Green,
        3 => Color::Cyan,
        2 => Color::Blue,
        1 => Color::Purple,
        _ => Color::Black,
    };
    let style = if priority >= 0 {
        color.bold()
    } else {
        color.bold().dimmed()
    };
    out.push_str(&style.paint(format!("P{}", priority)).to_string());
    out.push(' ');
}

fn fmt_budget(target_progress: i32, out: &mut String) {
    out.push_str(&Color::White.bold().paint("Target progress").to_string());
    out.push(' ');
    let style = if target_progress < 50 {
        Color::White.bold().dimmed()
    } else if target_progress < 80 {
        Color::Yellow.bold()
    } else {
        Color::Red.bold()
    };
    out.push_str(&style.paint(format!("{}%", target_progress)).to_string());
    out.push(' ');
}

fn fmt_due_date(
    due_date: DateTime<Utc>,
    context: &PrintingContext,
    out: &mut String,
) {
    let style = match calculate_urgency(context.now, due_date) {
        Urgency::Urgent => Color::Red.bold(),
        Urgency::Moderate => Color::Yellow.bold(),
        Urgency::Meh => Color::White.bold().dimmed(),
    };
    let desc = ::time_format::display_relative_time(
        context.now.with_timezone(&Local),
        due_date.with_timezone(&Local),
    );
    out.push_str(&style.paint(format!("Due {}", desc)).to_string());
    out.push(' ');
}

// If the task has deps, show a lock icon, followed by the number of incomplete
// deps and the number of total deps, as a fraction. E.g. if the task has 3
// deps, 2 of which are incomplete, show "🔓 2/3".
fn fmt_locks(incomplete: usize, total: usize, out: &mut String) {
    out.push_str(
        &Color::Red
            .paint(format!("🔒{}/{}", incomplete, total))
            .to_string(),
    );
    out.push(' ');
}

// If the task has adeps, show an unlock icon, followed by the number of
// unlockable adeps and the number of total adeps, as a fraction. E.g. if the
// task would unlock two adeps when it is completed, out of three total adeps,
// show "🔓2/3".
//
// If none of the adeps are unlockable, the first number is 0.
fn fmt_unlocks(unlockable: usize, total: usize, out: &mut String) {
    out.push_str(
        &Color::White
            .paint(format!("🔓{unlockable}/{total}"))
            .to_string(),
    );
    out.push(' ');
}

fn get_body(task: &PrintableTask, context: &PrintingContext) -> String {
    let mut body = String::new();
    if let Some(start_date) = task.start_date {
        fmt_snooze_date(start_date - context.now, &mut body);
    }
    if task.priority != 0 {
        fmt_priority(task.priority, &mut body);
    }
    if let Some(due_date) = task.due_date {
        fmt_due_date(due_date, context, &mut body);
        if let Some(budget) = task.budget {
            let target_progress =
                calculate_progress(context.now, due_date, budget);
            if (0..=100).contains(&target_progress) {
                fmt_budget(target_progress, &mut body)
            }
        }
    }
    let (incomplete, total) = task.deps_stats;
    if total > 0 {
        fmt_locks(incomplete, total, &mut body);
    }
    let (unlockable, total) = task.adeps_stats;
    if total > 0 {
        fmt_unlocks(unlockable, total, &mut body);
    }
    body.push_str(task.desc);
    body
}

fn get_subsequent_indent(
    task: &PrintableTask,
    context: &PrintingContext,
) -> String {
    let maybe_log_date_offset = if task.log_date.is_some() {
        LOG_DATE_OFFSET
    } else {
        0
    };
    let total_offset =
        context.max_index_digits + SELECTOR_OFFSET + maybe_log_date_offset;
    " ".repeat(total_offset)
}

impl<'a> Display for PrintableTaskWithContext<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start = get_initial_indent(self.task, self.context);
        let body = get_body(self.task, self.context);
        let subsequent_indent = get_subsequent_indent(self.task, self.context);
        f.write_str(&textwrap::fill(
            &body,
            textwrap::Options::new(self.context.width)
                .initial_indent(&start)
                .break_words(false)
                .subsequent_indent(&subsequent_indent),
        ))
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
        )
        .unwrap_or_default();
    }
    fn print_warning(&mut self, warning: &PrintableWarning) {
        writeln!(self.out, "{}", warning).unwrap_or_default();
    }
    fn print_error(&mut self, error: &PrintableError) {
        writeln!(self.out, "{}", error).unwrap_or_default();
    }
}
