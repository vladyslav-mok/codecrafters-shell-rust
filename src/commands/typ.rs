use super::{Command, CommandsRegistry, OutputOfCommand};

pub struct TypeCommand;

impl Command for TypeCommand {
    fn run(
        &self,
        args: &[&str],
        reg: &CommandsRegistry,
        _: &OutputOfCommand,
    ) -> Result<(), String> {
        let command_name = match args.first() {
            Some(arg) => arg,
            None => return Err("example usage: type <command name>".to_string()),
        };

        if let Some(command) = reg.get_command(command_name) {
            println!("{}", command.get_type_message());
        } else {
            println!("{}: not found", command_name);
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "type".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
