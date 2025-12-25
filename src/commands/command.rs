use std::io::Write;

use crate::commands::CommandRegistry;
use crate::error::ShellResult;

#[derive(Debug, PartialEq)]
pub enum ShellStatus {
    Continue,
    Exit,
}

pub trait Command {
    fn execute(
        &self,
        args: &[String],
        registry: &CommandRegistry,
        output: &mut dyn Write,
    ) -> ShellResult<ShellStatus>;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str {
        "shell builtin"
    }
}
