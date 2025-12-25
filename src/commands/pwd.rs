use std::io::Write;

use super::{Command, CommandRegistry, ShellStatus};
use crate::error::ShellResult;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(
        &self,
        _: &[String],
        _: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        let current_dir = std::env::current_dir()?;
        writeln!(output, "{}", current_dir.display())?;
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "pwd"
    }
}
