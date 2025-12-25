use codecrafters_shell::commands::{CommandRegistry, ShellStatus};
use codecrafters_shell::error::ShellResult;

#[cfg(test)]
mod echo_tests {
    use super::*;

    fn execute_echo(args: &[&str]) -> (String, ShellResult<ShellStatus>) {
        let registry = CommandRegistry::default();
        let echo_cmd = registry.get_builtin("echo").unwrap();
        let mut output = Vec::new();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let result = echo_cmd.execute(&args, &registry, &mut output);
        (String::from_utf8(output).unwrap(), result)
    }

    #[test]
    fn test_echo_simple() {
        let (output, result) = execute_echo(&["hello"]);
        assert!(result.is_ok());
        assert_eq!(output, "hello\n");
    }

    #[test]
    fn test_echo_multiple_args() {
        let (output, result) = execute_echo(&["hello", "world", "test"]);
        assert!(result.is_ok());
        assert_eq!(output, "hello world test\n");
    }

    #[test]
    fn test_echo_no_args() {
        let (output, result) = execute_echo(&[]);
        assert!(result.is_ok());
        assert_eq!(output, "\n");
    }

    #[test]
    fn test_echo_with_backslash() {
        let (output, result) = execute_echo(&["hello\\nworld"]);
        assert!(result.is_ok());
        assert_eq!(output, "hello\\nworld\n");
    }

    #[test]
    fn test_echo_with_single_quotes() {
        let (output, result) = execute_echo(&["'hello world'"]);
        assert!(result.is_ok());
        assert_eq!(output, "'hello world'\n");
    }

    #[test]
    fn test_echo_with_double_quotes() {
        let (output, result) = execute_echo(&["\"hello world\""]);
        assert!(result.is_ok());
        assert_eq!(output, "\"hello world\"\n");
    }
}

#[cfg(test)]
mod pwd_tests {
    use super::*;
    use std::env;

    fn execute_pwd() -> (String, ShellResult<ShellStatus>) {
        let registry = CommandRegistry::default();
        let pwd_cmd = registry.get_builtin("pwd").unwrap();
        let mut output = Vec::new();
        let result = pwd_cmd.execute(&[], &registry, &mut output);
        (String::from_utf8(output).unwrap(), result)
    }

    #[test]
    fn test_pwd_returns_current_directory() {
        let (output, result) = execute_pwd();
        assert!(result.is_ok());

        let expected = env::current_dir().unwrap();
        assert_eq!(output.trim(), expected.to_str().unwrap());
    }

    #[test]
    fn test_pwd_ends_with_newline() {
        let (output, _) = execute_pwd();
        assert!(output.ends_with('\n'));
    }
}

#[cfg(test)]
mod cd_tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use std::sync::Mutex;

    // Use a mutex to ensure CD tests run serially (since they modify global state)
    static CD_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn execute_cd(args: &[&str]) -> ShellResult<ShellStatus> {
        let registry = CommandRegistry::default();
        let cd_cmd = registry.get_builtin("cd").unwrap();
        let mut output = Vec::new();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        cd_cmd.execute(&args, &registry, &mut output)
    }

    #[test]
    fn test_cd_to_root() {
        let _lock = CD_TEST_LOCK.lock().unwrap();
        let original = env::current_dir().unwrap();

        let result = execute_cd(&["/"]);
        assert!(result.is_ok());
        assert_eq!(env::current_dir().unwrap(), PathBuf::from("/"));

        // Restore original directory
        env::set_current_dir(original).unwrap();
    }

    #[test]
    fn test_cd_to_tmp() {
        let _lock = CD_TEST_LOCK.lock().unwrap();
        let original = env::current_dir().unwrap();

        let result = execute_cd(&["/tmp"]);
        assert!(result.is_ok());
        assert_eq!(env::current_dir().unwrap(), PathBuf::from("/tmp"));

        // Restore original directory
        env::set_current_dir(original).unwrap();
    }

    #[test]
    fn test_cd_to_nonexistent() {
        let _lock = CD_TEST_LOCK.lock().unwrap();
        let original = env::current_dir().unwrap();

        let result = execute_cd(&["/nonexistent_directory_12345"]);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("No such file or directory"));

        // Verify we're still in original directory
        assert_eq!(env::current_dir().unwrap(), original);
    }

    #[test]
    fn test_cd_to_home() {
        let _lock = CD_TEST_LOCK.lock().unwrap();
        let original = env::current_dir().unwrap();

        let result = execute_cd(&["~"]);
        assert!(result.is_ok());

        let expected_home = env::var("HOME").unwrap();
        assert_eq!(env::current_dir().unwrap(), PathBuf::from(expected_home));

        // Restore original directory
        env::set_current_dir(original).unwrap();
    }

    #[test]
    fn test_cd_no_args_does_nothing() {
        let _lock = CD_TEST_LOCK.lock().unwrap();
        let original = env::current_dir().unwrap();

        let result = execute_cd(&[]);
        assert!(result.is_ok());

        // With no args, cd currently does nothing (stays in current directory)
        assert_eq!(env::current_dir().unwrap(), original);
    }
}

#[cfg(test)]
mod exit_tests {
    use super::*;

    fn execute_exit(args: &[&str]) -> ShellResult<ShellStatus> {
        let registry = CommandRegistry::default();
        let exit_cmd = registry.get_builtin("exit").unwrap();
        let mut output = Vec::new();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        exit_cmd.execute(&args, &registry, &mut output)
    }

    #[test]
    fn test_exit_returns_exit_status() {
        let result = execute_exit(&[]);
        assert!(result.is_ok());
        matches!(result.unwrap(), ShellStatus::Exit);
    }

    #[test]
    fn test_exit_with_code() {
        let result = execute_exit(&["0"]);
        assert!(result.is_ok());
        matches!(result.unwrap(), ShellStatus::Exit);
    }
}

#[cfg(test)]
mod type_tests {
    use super::*;

    fn execute_type(args: &[&str]) -> (String, ShellResult<ShellStatus>) {
        let registry = CommandRegistry::default();
        let type_cmd = registry.get_builtin("type").unwrap();
        let mut output = Vec::new();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let result = type_cmd.execute(&args, &registry, &mut output);
        (String::from_utf8(output).unwrap(), result)
    }

    #[test]
    fn test_type_builtin_echo() {
        let (output, result) = execute_type(&["echo"]);
        assert!(result.is_ok());
        assert_eq!(output, "echo is a shell builtin\n");
    }

    #[test]
    fn test_type_builtin_cd() {
        let (output, result) = execute_type(&["cd"]);
        assert!(result.is_ok());
        assert_eq!(output, "cd is a shell builtin\n");
    }

    #[test]
    fn test_type_builtin_pwd() {
        let (output, result) = execute_type(&["pwd"]);
        assert!(result.is_ok());
        assert_eq!(output, "pwd is a shell builtin\n");
    }

    #[test]
    fn test_type_builtin_exit() {
        let (output, result) = execute_type(&["exit"]);
        assert!(result.is_ok());
        assert_eq!(output, "exit is a shell builtin\n");
    }

    #[test]
    fn test_type_builtin_type() {
        let (output, result) = execute_type(&["type"]);
        assert!(result.is_ok());
        assert_eq!(output, "type is a shell builtin\n");
    }

    #[test]
    fn test_type_external_command() {
        let (output, result) = execute_type(&["ls"]);
        assert!(result.is_ok());
        assert!(output.starts_with("ls is "));
        assert!(output.contains("/ls"));
    }

    #[test]
    fn test_type_nonexistent_command() {
        let (_output, result) = execute_type(&["nonexistent_command_xyz"]);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("not found"));
    }

    #[test]
    fn test_type_multiple_commands() {
        let (output, result) = execute_type(&["echo", "cd", "ls"]);
        assert!(result.is_ok());
        assert!(output.contains("echo is a shell builtin"));
        assert!(output.contains("cd is a shell builtin"));
        assert!(output.contains("ls is "));
    }
}

#[cfg(test)]
mod history_tests {
    use super::*;

    fn execute_history(args: &[&str]) -> (String, ShellResult<ShellStatus>) {
        let registry = CommandRegistry::default();

        // Add some history entries
        registry.add_history_entry("echo hello");
        registry.add_history_entry("pwd");
        registry.add_history_entry("cd /tmp");

        let history_cmd = registry.get_builtin("history").unwrap();
        let mut output = Vec::new();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let result = history_cmd.execute(&args, &registry, &mut output);
        (String::from_utf8(output).unwrap(), result)
    }

    #[test]
    fn test_history_displays_entries() {
        let (output, result) = execute_history(&[]);
        assert!(result.is_ok());
        assert!(output.contains("echo hello"));
        assert!(output.contains("pwd"));
        assert!(output.contains("cd /tmp"));
    }

    #[test]
    fn test_history_has_line_numbers() {
        let (output, result) = execute_history(&[]);
        assert!(result.is_ok());

        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 3);

        // Check that lines are numbered (with padding)
        assert!(lines[0].contains("1"));
        assert!(lines[1].contains("2"));
        assert!(lines[2].contains("3"));

        // Check content is present
        assert!(output.contains("echo hello"));
        assert!(output.contains("pwd"));
        assert!(output.contains("cd /tmp"));
    }
}
