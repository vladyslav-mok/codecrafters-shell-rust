use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{env, fs};

use super::Command;
use super::{
    cd::CdCommand, echo::EchoCommand, exit::ExitCommand, history::HistoryCommand, pwd::PwdCommand,
    type_cmd::TypeCommand,
};
use crate::error::ShellResult;
use crate::files::open_file;

const EXECUTABLE_PERMISSION_BITS: u32 = 0o111;

/// Manages command history with support for loading from and saving to files
struct HistoryManager {
    entries: RefCell<Vec<String>>,
}

impl HistoryManager {
    fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
        }
    }

    fn add_entry(&self, cmd: &str) {
        self.entries.borrow_mut().push(cmd.to_string());
    }

    fn get_entries(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }

    fn load_from_file(&self, path: &Path) -> ShellResult<()> {
        let content = fs::read_to_string(path)?;
        let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
        self.entries.borrow_mut().append(&mut lines);
        Ok(())
    }

    fn write_to_file(&self, path: &Path, append: bool, is_exit: bool) -> ShellResult<()> {
        let mut file = open_file(path, append)?;
        let entries = self.entries.borrow();

        let start_index = Self::calculate_start_index(&entries, append, is_exit);

        for entry in entries.iter().skip(start_index) {
            writeln!(file, "{}", entry)?;
        }

        Ok(())
    }

    fn calculate_start_index(entries: &[String], append: bool, is_exit: bool) -> usize {
        if append && !is_exit {
            entries
                .iter()
                .rev()
                .skip(1)
                .position(|x| x.starts_with("history -a"))
                .map(|rev_index| entries.len() - 1 - rev_index)
                .unwrap_or(0)
        } else {
            0
        }
    }
}

struct PathScanner;

impl PathScanner {
    fn get_path_dirs() -> Vec<String> {
        env::var("PATH")
            .unwrap_or_default()
            .split(':')
            .map(|s| s.to_string())
            .collect()
    }

    fn is_executable(path: &PathBuf) -> bool {
        fs::metadata(path)
            .map(|m| m.permissions().mode() & EXECUTABLE_PERMISSION_BITS != 0)
            .unwrap_or(false)
    }

    fn scan_executables() -> HashMap<String, String> {
        let executables: Vec<(String, String)> = Self::get_path_dirs()
            .iter()
            .filter_map(|path_dir| fs::read_dir(path_dir).ok())
            .flat_map(|entries| entries.flatten())
            .filter_map(|entry| {
                let file_name = entry.file_name().into_string().ok()?;
                let full_path = entry.path();

                if Self::is_executable(&full_path) {
                    let path_str = full_path.to_str()?.to_string();
                    Some((file_name, path_str))
                } else {
                    None
                }
            })
            .collect();

        let mut map = HashMap::new();
        for (name, path) in executables {
            map.entry(name).or_insert(path);
        }
        map
    }

    fn find_executable(command: &str) -> Option<String> {
        Self::get_path_dirs()
            .iter()
            .map(|path_dir| PathBuf::from(path_dir).join(command))
            .find(Self::is_executable)
            .and_then(|path| path.to_str().map(|s| s.to_string()))
    }
}

pub struct CommandRegistry {
    pub builtins: HashMap<String, Box<dyn Command>>,
    pub executables: HashMap<String, String>,
    history: HistoryManager,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            builtins: HashMap::new(),
            executables: HashMap::new(),
            history: HistoryManager::new(),
        }
    }

    pub fn get_builtin(&self, name: &str) -> Option<&dyn Command> {
        self.builtins.get(name).map(|b| b.as_ref())
    }

    pub fn get_command_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.builtins.keys().cloned().collect();
        names.extend(self.executables.keys().cloned());

        names.sort();
        names.dedup();

        names
    }

    fn register_builtin(&mut self, command: Box<dyn Command>) {
        self.builtins
            .insert(command.get_name().to_string(), command);
    }

    pub fn add_history_entry(&self, cmd: &str) {
        self.history.add_entry(cmd);
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.get_entries()
    }

    pub fn get_histfile_path() -> Option<PathBuf> {
        env::var("HISTFILE")
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
    }

    pub fn load_history_from_file(&self, path: &Path) -> ShellResult<()> {
        self.history.load_from_file(path)
    }

    pub fn write_history_to_file(
        &self,
        path: &Path,
        append: bool,
        is_exit: bool,
    ) -> ShellResult<()> {
        self.history.write_to_file(path, append, is_exit)
    }

    fn scan_path_executables(&mut self) {
        self.executables = PathScanner::scan_executables();
    }

    pub fn get_executable_path(&self, command: &str) -> Option<String> {
        PathScanner::find_executable(command)
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        let mut registry = CommandRegistry::new();
        registry.register_builtin(Box::new(TypeCommand));
        registry.register_builtin(Box::new(EchoCommand));
        registry.register_builtin(Box::new(ExitCommand));
        registry.register_builtin(Box::new(PwdCommand));
        registry.register_builtin(Box::new(CdCommand));
        registry.register_builtin(Box::new(HistoryCommand));

        registry.scan_path_executables();

        registry
    }
}
