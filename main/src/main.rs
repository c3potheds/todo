use clap::Parser;
use todo_cli::Options;
use todo_runner::TodoResult;

fn main() -> TodoResult {
    todo_runner::run(todo_builtin_apps::App::new(Options::parse()))
}
