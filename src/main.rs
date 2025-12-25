use rustyline::{CompletionType, Config, EditMode, Editor, error::ReadlineError};

use commands::{CommandRegistry, ShellExecutor, ShellStatus};
use shell::Shell;

mod commands;
mod error;
mod files;
mod parser;
mod shell;

const EXIT_INITIALIZATION_ERROR: i32 = 1;

const SHELL_PROMPT: &str = "$ ";

fn main() {
    let registry = CommandRegistry::default();
    let command_names = registry.get_command_names();
    let helper = Shell::new(command_names);
    let executor = ShellExecutor::new(&registry);

    if let Some(histfile) = CommandRegistry::get_histfile_path() {
        let _ = registry.load_history_from_file(&histfile);
    }

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let mut editor = Editor::<Shell, _>::with_config(config).unwrap_or_else(|e| {
        eprintln!("Failed to initialize editor: {}", e);
        std::process::exit(EXIT_INITIALIZATION_ERROR);
    });
    editor.set_helper(Some(helper));

    loop {
        let readline = editor.readline(SHELL_PROMPT);
        match readline {
            Ok(line) => {
                registry.add_history_entry(&line);
                editor.add_history_entry(line.as_str()).ok();

                let commands = parser::parse_input(line.as_str());

                if commands.is_empty() {
                    continue;
                }

                match executor.run(&commands) {
                    Ok(ShellStatus::Exit) => break,
                    Ok(ShellStatus::Continue) => continue,
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}
