use std::io::Write;

use super::{Command, CommandRegistry, ShellStatus};
use crate::error::ShellResult;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(
        &self,
        _: &[String],
        registry: &CommandRegistry,
        _: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        if let Some(histfile) = CommandRegistry::get_histfile_path() {
            let _ = registry.write_history_to_file(&histfile, false, false);
        }
        Ok(ShellStatus::Exit)
    }

    fn get_name(&self) -> &str {
        "exit"
    }
}
