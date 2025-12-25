use std::io::Write;
use std::path::Path;

use super::{Command, CommandRegistry, ShellStatus};
use crate::error::{ShellError, ShellResult};

const HISTORY_LINE_NUMBER_WIDTH: usize = 5;

pub struct HistoryCommand;

impl Command for HistoryCommand {
    fn execute(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        match args.first().map(|s| s.as_str()) {
            Some("-w") => {
                let path = args.get(1).ok_or_else(|| ShellError::HistoryArgRequired {
                    flag: "-w".to_string(),
                })?;
                registry.write_history_to_file(Path::new(path), false, false)?;
                Ok(ShellStatus::Continue)
            }

            Some("-a") => {
                let path = args.get(1).ok_or_else(|| ShellError::HistoryArgRequired {
                    flag: "-a".to_string(),
                })?;
                registry.write_history_to_file(Path::new(path), true, false)?;
                Ok(ShellStatus::Continue)
            }

            Some("-r") => {
                let path = args.get(1).ok_or_else(|| ShellError::HistoryArgRequired {
                    flag: "-r".to_string(),
                })?;
                registry.load_history_from_file(Path::new(path))?;
                Ok(ShellStatus::Continue)
            }

            _ => self.list_history(args, registry, output),
        }
    }

    fn get_name(&self) -> &str {
        "history"
    }
}

impl HistoryCommand {
    fn list_history(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        let history = registry.get_history();

        let limit = match args.first() {
            Some(arg) => arg
                .parse::<usize>()
                .map_err(|_| ShellError::HistoryInvalidArg {
                    arg: arg.to_string(),
                })?,
            None => history.len(),
        };

        let start_index = history.len().saturating_sub(limit);

        for (i, entry) in history.iter().enumerate().skip(start_index) {
            writeln!(
                output,
                "{:>width$}  {}",
                i + 1,
                entry,
                width = HISTORY_LINE_NUMBER_WIDTH
            )?;
        }

        Ok(ShellStatus::Continue)
    }
}
