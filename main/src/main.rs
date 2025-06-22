use clap::Parser;
use todo_cli::{Options, SubCommand}; // Added SubCommand
use todo_runner::TodoResult;
use todo_app::interactive; // Added this
use todo_printing::SimpleTodoPrinter; // For placeholder
use todo_model::TodoList; // For placeholder

fn main() -> TodoResult {
    let options = Options::parse();
    match options.cmd {
        Some(SubCommand::Interactive(_)) => {
            // This is a placeholder.
            // The actual interactive mode will need to load the TodoList
            // and set up the terminal.
            // For now, let's just print a message.
            // println!("Interactive mode coming soon!");
            // Or, call the placeholder from the app crate:
            let mut todo_list = TodoList::default(); // Placeholder
            match interactive::run_interactive_ui(&mut todo_list) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Interactive mode error: {}", e);
                    // Decide on an appropriate exit strategy for errors from interactive mode
                    std::process::exit(1);
                }
            }
        }
        _ => {
            // Original logic for non-interactive commands
            todo_runner::run(todo_builtin_apps::App::new(options))
        }
    }
}
