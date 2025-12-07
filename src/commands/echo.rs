use std::io::Write;
use std::{fs::File, path::Path};

use super::{Command, CommandsRegistry, OutputOfCommand};

pub struct EchoCommand;

impl Command for EchoCommand {
    fn run(
        &self,
        args: &[&str],
        _: &CommandsRegistry,
        output_of_command: &OutputOfCommand,
    ) -> Result<(), String> {
        let output_str = args.join(" ");

        fn append<P: AsRef<Path>, C: AsRef<str>>(path: P, contents: C) -> Result<(), String> {
            let mut f = File::options()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| e.to_string())?;
            writeln!(&mut f, "{}", contents.as_ref()).map_err(|e| e.to_string())?;
            Ok(())
        }

        if let Some(path) = output_of_command.output_create.as_ref() {
            std::fs::write(path, format!("{}\n", output_str)).map_err(|e| e.to_string())?;
        } else if let Some(err_path) = output_of_command.error_output_create.as_ref() {
            std::fs::write(err_path, "").map_err(|e| e.to_string())?;
            println!("{}", output_str);
        } else if let Some(path) = output_of_command.output_append.as_ref() {
            append(path, &output_str)?;
        } else if let Some(err_path) = output_of_command.error_output_append.as_ref() {
            append(err_path, "")?;
            println!("{}", output_str);
        } else {
            println!("{}", output_str);
        }
        Ok(())
    }

    fn get_name(&self) -> String {
        "echo".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
