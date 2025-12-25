use std::env;
use std::io::Write;
use std::path::Path;

use super::{Command, CommandRegistry, ShellStatus};
use crate::error::{ShellError, ShellResult};

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(
        &self,
        args: &[String],
        _: &CommandRegistry,
        _: &mut dyn Write,
    ) -> ShellResult<ShellStatus> {
        if args.is_empty() {
            return Ok(ShellStatus::Continue);
        }

        if args[0] == "~" {
            let home_path: String = env::var("HOME").unwrap_or_default();
            env::set_current_dir(&home_path).map_err(|_| ShellError::DirectoryNotFound {
                path: home_path.clone(),
            })?;
            return Ok(ShellStatus::Continue);
        }

        let new_dir = &args[0];
        let root = Path::new(new_dir);

        env::set_current_dir(root).map_err(|_| ShellError::DirectoryNotFound {
            path: new_dir.clone(),
        })?;

        Ok(ShellStatus::Continue)
    }

    fn get_name(&self) -> &str {
        "cd"
    }
}
