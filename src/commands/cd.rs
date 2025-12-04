use super::{Command, CommandsRegistry};

pub struct CdCommand;

impl Command for CdCommand {
    fn run(
        &self,
        args: Vec<&str>,
        _: &CommandsRegistry,
        _: Option<String>,
        _: Option<String>,
    ) -> Result<(), String> {
        if args.is_empty() {
            return Err("Usage: cd <directory>".to_string());
        }
        let path = if args[0] == "~" {
            match std::env::var("HOME") {
                Ok(home) => home,
                Err(_) => return Err("cd: HOME not set".to_string()),
            }
        } else {
            args[0].to_string()
        };
        if std::env::set_current_dir(path).is_ok() {
            Ok(())
        } else {
            Err(format!("cd: {}: No such file or directory", args[0]))
        }
    }

    fn get_name(&self) -> String {
        "cd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
