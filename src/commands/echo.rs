use std::io::Write;

use super::{Command, CommandRegistry, ShellStatus};
use crate::error::ShellResult;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(
        &self,
        args: &[String],
        _: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        writeln!(output, "{}", args.join(" "))?;
        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "echo"
    }
}
