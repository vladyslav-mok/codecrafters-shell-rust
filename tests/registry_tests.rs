use codecrafters_shell::commands::CommandRegistry;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

#[cfg(test)]
mod builtin_lookup_tests {
    use super::*;

    #[test]
    fn test_registry_has_echo_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("echo").is_some());
    }

    #[test]
    fn test_registry_has_pwd_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("pwd").is_some());
    }

    #[test]
    fn test_registry_has_cd_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("cd").is_some());
    }

    #[test]
    fn test_registry_has_exit_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("exit").is_some());
    }

    #[test]
    fn test_registry_has_type_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("type").is_some());
    }

    #[test]
    fn test_registry_has_history_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("history").is_some());
    }

    #[test]
    fn test_registry_nonexistent_builtin() {
        let registry = CommandRegistry::default();
        assert!(registry.get_builtin("nonexistent").is_none());
    }
}

#[cfg(test)]
mod executable_lookup_tests {
    use super::*;

    #[test]
    fn test_get_executable_path_for_ls() {
        let registry = CommandRegistry::default();
        let path = registry.get_executable_path("ls");
        assert!(path.is_some());
        assert!(path.unwrap().contains("/ls"));
    }

    #[test]
    fn test_get_executable_path_for_cat() {
        let registry = CommandRegistry::default();
        let path = registry.get_executable_path("cat");
        assert!(path.is_some());
        assert!(path.unwrap().contains("/cat"));
    }

    #[test]
    fn test_get_executable_path_nonexistent() {
        let registry = CommandRegistry::default();
        let path = registry.get_executable_path("nonexistent_command_xyz");
        assert!(path.is_none());
    }

    #[test]
    fn test_executable_path_is_absolute() {
        let registry = CommandRegistry::default();
        let path = registry.get_executable_path("ls").unwrap();
        assert!(path.starts_with('/'));
    }

    #[test]
    fn test_custom_path_executable() {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("testbin");
        fs::create_dir(&bin_dir).unwrap();

        // Create a test executable
        let exe_path = bin_dir.join("testcmd");
        fs::write(&exe_path, "#!/bin/sh\necho test").unwrap();

        // Make it executable
        let mut perms = fs::metadata(&exe_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&exe_path, perms).unwrap();

        // Temporarily modify PATH
        let original_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}:{}", bin_dir.to_str().unwrap(), original_path),
            );
        }

        let registry = CommandRegistry::default();
        let found_path = registry.get_executable_path("testcmd");

        // Restore PATH
        unsafe {
            env::set_var("PATH", original_path);
        }

        assert!(found_path.is_some());
        assert_eq!(found_path.unwrap(), exe_path.to_str().unwrap());
    }

    #[test]
    fn test_non_executable_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let bin_dir = temp_dir.path().join("testbin");
        fs::create_dir(&bin_dir).unwrap();

        // Create a non-executable file
        let file_path = bin_dir.join("notexecutable");
        fs::write(&file_path, "test").unwrap();

        // Temporarily modify PATH
        let original_path = env::var("PATH").unwrap_or_default();
        unsafe {
            env::set_var(
                "PATH",
                format!("{}:{}", bin_dir.to_str().unwrap(), original_path),
            );
        }

        let registry = CommandRegistry::default();
        let found_path = registry.get_executable_path("notexecutable");

        // Restore PATH
        unsafe {
            env::set_var("PATH", original_path);
        }

        assert!(found_path.is_none());
    }
}

#[cfg(test)]
mod command_names_tests {
    use super::*;

    #[test]
    fn test_get_command_names_includes_builtins() {
        let registry = CommandRegistry::default();
        let names = registry.get_command_names();

        assert!(names.contains(&"echo".to_string()));
        assert!(names.contains(&"pwd".to_string()));
        assert!(names.contains(&"cd".to_string()));
        assert!(names.contains(&"exit".to_string()));
        assert!(names.contains(&"type".to_string()));
    }

    #[test]
    fn test_get_command_names_includes_executables() {
        let registry = CommandRegistry::default();
        let names = registry.get_command_names();

        assert!(names.contains(&"ls".to_string()));
        assert!(names.contains(&"cat".to_string()));
    }

    #[test]
    fn test_get_command_names_sorted() {
        let registry = CommandRegistry::default();
        let names = registry.get_command_names();

        let mut sorted_names = names.clone();
        sorted_names.sort();

        assert_eq!(names, sorted_names);
    }

    #[test]
    fn test_get_command_names_no_duplicates() {
        let registry = CommandRegistry::default();
        let names = registry.get_command_names();

        let mut deduped_names = names.clone();
        deduped_names.dedup();

        assert_eq!(names.len(), deduped_names.len());
    }
}

#[cfg(test)]
mod history_tests {
    use super::*;

    #[test]
    fn test_add_history_entry() {
        let registry = CommandRegistry::default();

        registry.add_history_entry("echo hello");
        registry.add_history_entry("pwd");

        let history = registry.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "echo hello");
        assert_eq!(history[1], "pwd");
    }

    #[test]
    fn test_empty_history() {
        let registry = CommandRegistry::default();
        let history = registry.get_history();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_history_preserves_order() {
        let registry = CommandRegistry::default();

        registry.add_history_entry("first");
        registry.add_history_entry("second");
        registry.add_history_entry("third");

        let history = registry.get_history();
        assert_eq!(history[0], "first");
        assert_eq!(history[1], "second");
        assert_eq!(history[2], "third");
    }

    #[test]
    fn test_load_history_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let history_file = temp_dir.path().join("history.txt");

        fs::write(&history_file, "command1\ncommand2\ncommand3\n").unwrap();

        let registry = CommandRegistry::default();
        let result = registry.load_history_from_file(&history_file);

        assert!(result.is_ok());

        let history = registry.get_history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "command1");
        assert_eq!(history[1], "command2");
        assert_eq!(history[2], "command3");
    }

    #[test]
    fn test_load_history_from_nonexistent_file() {
        let registry = CommandRegistry::default();
        let result =
            registry.load_history_from_file(std::path::Path::new("/nonexistent/history.txt"));

        assert!(result.is_err());
    }

    #[test]
    fn test_write_history_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let history_file = temp_dir.path().join("history.txt");

        let registry = CommandRegistry::default();
        registry.add_history_entry("cmd1");
        registry.add_history_entry("cmd2");

        let result = registry.write_history_to_file(&history_file, false, false);

        assert!(result.is_ok());

        let content = fs::read_to_string(&history_file).unwrap();
        assert_eq!(content, "cmd1\ncmd2\n");
    }

    #[test]
    fn test_write_history_append() {
        let temp_dir = TempDir::new().unwrap();
        let history_file = temp_dir.path().join("history.txt");

        // Create initial file
        fs::write(&history_file, "existing1\nexisting2\n").unwrap();

        let registry = CommandRegistry::default();
        registry.add_history_entry("new1");
        registry.add_history_entry("new2");

        let result = registry.write_history_to_file(&history_file, true, false);

        assert!(result.is_ok());

        let content = fs::read_to_string(&history_file).unwrap();
        assert_eq!(content, "existing1\nexisting2\nnew1\nnew2\n");
    }

    #[test]
    fn test_write_history_truncate() {
        let temp_dir = TempDir::new().unwrap();
        let history_file = temp_dir.path().join("history.txt");

        // Create initial file
        fs::write(&history_file, "existing1\nexisting2\n").unwrap();

        let registry = CommandRegistry::default();
        registry.add_history_entry("new1");
        registry.add_history_entry("new2");

        let result = registry.write_history_to_file(&history_file, false, false);

        assert!(result.is_ok());

        let content = fs::read_to_string(&history_file).unwrap();
        assert_eq!(content, "new1\nnew2\n");
    }
}
