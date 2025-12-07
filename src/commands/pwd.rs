use super::{Command, CommandsRegistry, OutputOfCommand};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(&self, _: &[&str], _: &CommandsRegistry, _: &OutputOfCommand) -> Result<(), String> {
        match std::env::current_dir() {
            Ok(cwd) => {
                println!("{}", cwd.display());
                Ok(())
            }
            Err(_) => Err("pwd: failed to get current directory".to_string()),
        }
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
