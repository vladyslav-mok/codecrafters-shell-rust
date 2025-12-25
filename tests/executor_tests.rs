use codecrafters_shell::commands::{CommandRegistry, ShellExecutor, ShellStatus};
use codecrafters_shell::parser::parse_input;
use std::fs;
use tempfile::TempDir;

fn setup_test_env() -> TempDir {
    TempDir::new().unwrap()
}

#[cfg(test)]
mod builtin_execution_tests {
    use super::*;

    #[test]
    fn test_execute_echo() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("echo hello world");
        let result = executor.run(&commands);

        assert!(result.is_ok());
        matches!(result.unwrap(), ShellStatus::Continue);
    }

    #[test]
    fn test_execute_pwd() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("pwd");
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_exit() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("exit");
        let result = executor.run(&commands);

        assert!(result.is_ok());
        matches!(result.unwrap(), ShellStatus::Exit);
    }
}

#[cfg(test)]
mod external_execution_tests {
    use super::*;

    #[test]
    fn test_execute_external_command() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("true");
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_nonexistent_command() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("nonexistent_command_xyz");
        let result = executor.run(&commands);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("command not found"));
    }
}

#[cfg(test)]
mod redirect_tests {
    use super::*;

    #[test]
    fn test_stdout_redirect_truncate() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.txt");
        let output_path = output_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input(&format!("echo hello > {}", output_path));
        let result = executor.run(&commands);

        assert!(result.is_ok());

        let content = fs::read_to_string(&output_file).unwrap();
        assert_eq!(content, "hello\n");
    }

    #[test]
    fn test_stdout_redirect_append() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.txt");
        let output_path = output_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        // First write
        let commands = parse_input(&format!("echo first >> {}", output_path));
        executor.run(&commands).unwrap();

        // Second write (append)
        let commands = parse_input(&format!("echo second >> {}", output_path));
        executor.run(&commands).unwrap();

        let content = fs::read_to_string(&output_file).unwrap();
        assert_eq!(content, "first\nsecond\n");
    }

    #[test]
    fn test_stdout_redirect_overwrites() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.txt");
        let output_path = output_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        // First write
        let commands = parse_input(&format!("echo first > {}", output_path));
        executor.run(&commands).unwrap();

        // Second write (overwrite)
        let commands = parse_input(&format!("echo second > {}", output_path));
        executor.run(&commands).unwrap();

        let content = fs::read_to_string(&output_file).unwrap();
        assert_eq!(content, "second\n");
    }

    #[test]
    fn test_stderr_redirect() {
        let temp_dir = setup_test_env();
        let error_file = temp_dir.path().join("error.txt");
        let error_path = error_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        // Use a command that writes to stderr (cat with nonexistent file)
        let commands = parse_input(&format!("cat /nonexistent_file_xyz 2> {}", error_path));
        let result = executor.run(&commands);

        // Command should execute (even if it fails)
        assert!(result.is_ok());

        // Error file should exist and contain error message
        assert!(error_file.exists());
        let content = fs::read_to_string(&error_file).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_both_stdout_and_stderr_redirect() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.txt");
        let error_file = temp_dir.path().join("error.txt");
        let output_path = output_file.to_str().unwrap();
        let error_path = error_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input(&format!(
            "cat /nonexistent_file_xyz > {} 2> {}",
            output_path, error_path
        ));
        executor.run(&commands).ok();

        // Both files should exist
        assert!(output_file.exists());
        assert!(error_file.exists());
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_simple_pipeline_builtin_to_external() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("echo hello | cat");
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }

    #[test]
    fn test_simple_pipeline_external_to_external() {
        let temp_dir = setup_test_env();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "line1\nline2\nline3\n").unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input(&format!("cat {} | wc -l", test_file.to_str().unwrap()));
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }

    #[test]
    fn test_three_stage_pipeline() {
        let temp_dir = setup_test_env();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "apple\nbanana\napple\ncherry\napple\n").unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input(&format!(
            "cat {} | grep apple | wc -l",
            test_file.to_str().unwrap()
        ));
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_with_redirect() {
        let temp_dir = setup_test_env();
        let output_file = temp_dir.path().join("output.txt");
        let output_path = output_file.to_str().unwrap();

        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input(&format!("echo hello | cat > {}", output_path));
        let result = executor.run(&commands);

        assert!(result.is_ok());

        let content = fs::read_to_string(&output_file).unwrap();
        assert_eq!(content, "hello\n");
    }

    #[test]
    fn test_pipeline_builtin_to_builtin() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("echo test | echo hello");
        let result = executor.run(&commands);

        // This should work (second echo ignores input)
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod empty_command_tests {
    use super::*;

    #[test]
    fn test_empty_command_list() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("");
        let result = executor.run(&commands);

        assert!(result.is_ok());
        matches!(result.unwrap(), ShellStatus::Continue);
    }

    #[test]
    fn test_whitespace_only() {
        let registry = CommandRegistry::default();
        let executor = ShellExecutor::new(&registry);

        let commands = parse_input("   \t  ");
        let result = executor.run(&commands);

        assert!(result.is_ok());
    }
}
