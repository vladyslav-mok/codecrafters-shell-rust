use std::io::Write;

use super::CommandRegistry;
use super::{Command, ShellStatus};
use crate::error::{ShellError, ShellResult};

pub struct TypeCommand;

impl Command for TypeCommand {
    fn execute(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        if args.is_empty() {
            return Ok(ShellStatus::Continue);
        }

        for arg in args {
            if let Some(command) = registry.get_builtin(arg) {
                writeln!(output, "{} is a {}", arg, command.get_type())?;
            } else if let Some(executable_path) = registry.get_executable_path(arg) {
                writeln!(output, "{} is {}", arg, executable_path)?;
            } else {
                return Err(ShellError::TypeNotFound(arg.clone()));
            }
        }

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "type"
    }
}
