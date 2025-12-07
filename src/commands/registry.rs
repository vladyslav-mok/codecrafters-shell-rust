use std::{collections::HashMap, fs, os::unix::fs::PermissionsExt, path::Path};

use super::{
    CdCommand, Command, EchoCommand, ExitCommand, ExternalCommand, PwdCommand, TypeCommand,
};

pub struct CommandsRegistry {
    builtin: HashMap<String, Box<dyn Command>>,
    external: HashMap<String, Box<dyn Command>>,
}

impl CommandsRegistry {
    pub fn new() -> Self {
        Self {
            builtin: HashMap::new(),
            external: HashMap::new(),
        }
    }

    pub fn register_builtin(&mut self, command: Box<dyn Command>) {
        assert!(!self.builtin.contains_key(&command.get_name()));
        self.builtin.insert(command.get_name(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&dyn Command> {
        match self.builtin.get(name) {
            Some(command) => Some(command.as_ref()),
            None => match self.external.get(name) {
                Some(command) => Some(command.as_ref()),
                None => None,
            },
        }
    }

    pub fn register_external(&mut self) {
        if let Ok(path) = std::env::var("PATH") {
            let paths = path.split(':').collect::<Vec<&str>>();

            for path in paths {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if !is_executable(&entry.path()) {
                            continue;
                        }
                        let name = entry.file_name().to_str().unwrap().to_string();
                        if self.external.contains_key(&name) {
                            continue;
                        }

                        let external_command =
                            ExternalCommand::new(name, entry.path().to_str().unwrap().to_string());

                        self.external
                            .insert(external_command.get_name(), Box::new(external_command));
                    }
                }
            }
        }
    }
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            let mode = metadata.permissions().mode();
            mode & 0o111 != 0
        } else {
            false
        }
    } else {
        false
    }
}

impl Default for CommandsRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        registry.register_builtin(Box::new(ExitCommand));
        registry.register_builtin(Box::new(EchoCommand));
        registry.register_builtin(Box::new(TypeCommand));
        registry.register_builtin(Box::new(PwdCommand));
        registry.register_builtin(Box::new(CdCommand));

        registry.register_external();

        registry
    }
}
