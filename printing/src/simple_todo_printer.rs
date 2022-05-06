use {
    crate::{
        format_util::format_number, Plicit, PrintableError, PrintableTask,
        PrintableWarning, TodoPrinter,
    },
    ansi_term::Color,
    chrono::{DateTime, Duration, Local, Utc},
    std::{
        fmt,
        fmt::{Display, Formatter},
        io::Write,
    },
};

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

fn fmt_priority(priority: &Plicit<i32>, out: &mut String) {
    let (priority, implicit) = match priority {
        Plicit::Explicit(priority) => (*priority, false),
        Plicit::Implicit(priority) => (*priority, true),
    };
    let color = match priority.abs() {
        6..=i32::MAX => Color::Red,
        5 => Color::Yellow,
        4 => Color::Green,
        3 => Color::Cyan,
        2 => Color::Blue,
        1 => Color::Purple,
        _ => Color::Black,
    };
    let mut style = if priority >= 0 {
        color.bold()
    } else {
        color.bold().dimmed()
    };
    if implicit {
        style = style.italic();
    }
    out.push_str(&style.paint(format!("P{}", priority)).to_string());
    out.push(' ');
}

fn fmt_due_date(
    due_date: &Plicit<DateTime<Utc>>,
    context: &PrintingContext,
    out: &mut String,
) {
    let (due_date, implicit) = match due_date {
        Plicit::Explicit(due_date) => (*due_date, false),
        Plicit::Implicit(due_date) => (*due_date, true),
    };
    let mut style = match calculate_urgency(context.now, due_date) {
        Urgency::Urgent => Color::Red.bold(),
        Urgency::Moderate => Color::Yellow.bold(),
        Urgency::Meh => Color::White.bold().dimmed(),
    };
    if implicit {
        style = style.italic();
    }
    let desc = ::time_format::display_relative_time(
        context.now.with_timezone(&Local),
        due_date.with_timezone(&Local),
    );
    out.push_str(&style.paint(format!("Due {}", desc)).to_string());
    out.push(' ');
}

fn fmt_punctuality(punctuality: Duration, out: &mut String) {
    let (style, suffix, abs_punctuality) =
        if punctuality > chrono::Duration::zero() {
            (Color::Red.bold(), "late", punctuality)
        } else {
            (Color::Green.bold(), "early", -punctuality)
        };
    let desc = ::time_format::format_duration_laconic(abs_punctuality);
    out.push_str(&style.paint(format!("Done {} {}", desc, suffix)).to_string());
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
    if let Some(priority) = &task.priority {
        fmt_priority(priority, &mut body);
    }
    let (incomplete, total) = task.deps_stats;
    if total > 0 {
        fmt_locks(incomplete, total, &mut body);
    }
    let (unlockable, total) = task.adeps_stats;
    if total > 0 {
        fmt_unlocks(unlockable, total, &mut body);
    }
    if let Some(due_date) = &task.due_date {
        fmt_due_date(due_date, context, &mut body);
    }
    if let Some(punctuality) = task.punctuality {
        fmt_punctuality(punctuality, &mut body);
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
        if body.is_empty() {
            return f.write_str(start.trim_end());
        }
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
    fn print_task<'a>(&mut self, task: &PrintableTask<'a>) {
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
