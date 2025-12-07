use std::{fs::File, io::Write, path::Path};

use super::{Command, CommandsRegistry, OutputOfCommand};

#[derive(Debug)]
pub struct ExternalCommand {
    name: String,
    path: String,
}

impl ExternalCommand {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}

impl Command for ExternalCommand {
    fn run(
        &self,
        args: &[&str],
        _: &CommandsRegistry,
        output_of_command: &OutputOfCommand,
    ) -> Result<(), String> {
        let output = std::process::Command::new(&self.name)
            .args(args)
            .output()
            .map_err(|err| err.to_string())?;

        fn append<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), String> {
            File::options()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| e.to_string())?
                .write_all(contents.as_ref())
                .map_err(|e| e.to_string())?;
            Ok(())
        }

        if let Some(path) = output_of_command.output_create.as_ref() {
            std::fs::write(path, output.stdout).map_err(|err| err.to_string())?;
        } else if let Some(path) = output_of_command.output_append.as_ref() {
            append(path, output.stdout)?;
        } else {
            std::io::stdout()
                .write_all(&output.stdout)
                .map_err(|err| err.to_string())?;
        }

        if let Some(err_path) = output_of_command.error_output_create.as_ref() {
            std::fs::write(err_path, output.stderr).map_err(|err| err.to_string())?;
        } else if let Some(err_path) = output_of_command.error_output_append.as_ref() {
            append(err_path, output.stderr)?;
        } else {
            std::io::stderr()
                .write_all(&output.stderr)
                .map_err(|err| err.to_string())?;
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type_message(&self) -> String {
        format!("{} is {}", self.get_name(), self.path)
    }
}
