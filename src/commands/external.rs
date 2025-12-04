use std::io::Write;

use super::{Command, CommandsRegistry};

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
        args: Vec<&str>,
        _: &CommandsRegistry,
        redirect_path: Option<String>,
        redirect_error_path: Option<String>,
    ) -> Result<(), String> {
        let output = std::process::Command::new(&self.name)
            .args(args)
            .output()
            .map_err(|err| err.to_string())?;
        if let Some(path) = redirect_path {
            std::fs::write(path, output.stdout).map_err(|err| err.to_string())?;
        } else {
            std::io::stdout()
                .write_all(&output.stdout)
                .map_err(|err| err.to_string())?;
        }
        if let Some(err_path) = redirect_error_path {
            std::fs::write(err_path, output.stderr).map_err(|err| err.to_string())?;
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
