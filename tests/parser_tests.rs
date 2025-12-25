use std::path::PathBuf;

use codecrafters_shell::parser::{parse_input, tokenize_input};

#[cfg(test)]
mod tokenize_tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let result = tokenize_input("echo hello");
        assert_eq!(result, vec!["echo", "hello"]);
    }

    #[test]
    fn test_tokenize_multiple_spaces() {
        let result = tokenize_input("echo    hello    world");
        assert_eq!(result, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_tokenize_empty_string() {
        let result = tokenize_input("");
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let result = tokenize_input("   \t  \n  ");
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_tokenize_single_quotes_preserve_spaces() {
        let result = tokenize_input("echo 'hello world'");
        assert_eq!(result, vec!["echo", "hello world"]);
    }

    #[test]
    fn test_tokenize_single_quotes_preserve_special_chars() {
        let result = tokenize_input("echo 'hello $USER `date`'");
        assert_eq!(result, vec!["echo", "hello $USER `date`"]);
    }

    #[test]
    fn test_tokenize_double_quotes_preserve_spaces() {
        let result = tokenize_input(r#"echo "hello world""#);
        assert_eq!(result, vec!["echo", "hello world"]);
    }

    #[test]
    fn test_tokenize_double_quotes_with_escaped_quote() {
        let result = tokenize_input(r#"echo "hello \"world\"""#);
        assert_eq!(result, vec!["echo", r#"hello "world""#]);
    }

    #[test]
    fn test_tokenize_double_quotes_with_escaped_backslash() {
        let result = tokenize_input(r#"echo "hello\\world""#);
        assert_eq!(result, vec!["echo", r"hello\world"]);
    }

    #[test]
    fn test_tokenize_backslash_escape_space() {
        let result = tokenize_input(r"echo hello\ world");
        assert_eq!(result, vec!["echo", "hello world"]);
    }

    #[test]
    fn test_tokenize_backslash_escape_in_unquoted() {
        let result = tokenize_input(r"echo \$USER");
        assert_eq!(result, vec!["echo", "$USER"]);
    }

    #[test]
    fn test_tokenize_backslash_not_special_in_single_quotes() {
        let result = tokenize_input(r"echo 'hello\nworld'");
        assert_eq!(result, vec!["echo", r"hello\nworld"]);
    }

    #[test]
    fn test_tokenize_pipe_operator() {
        let result = tokenize_input("echo hello | cat");
        assert_eq!(result, vec!["echo", "hello", "|", "cat"]);
    }

    #[test]
    fn test_tokenize_pipe_in_quotes() {
        let result = tokenize_input(r#"echo "hello|world""#);
        assert_eq!(result, vec!["echo", "hello|world"]);
    }

    #[test]
    fn test_tokenize_multiple_pipes() {
        let result = tokenize_input("cat file | grep pattern | wc -l");
        assert_eq!(
            result,
            vec!["cat", "file", "|", "grep", "pattern", "|", "wc", "-l"]
        );
    }

    #[test]
    fn test_tokenize_mixed_quotes() {
        let result = tokenize_input(r#"echo 'single' "double" unquoted"#);
        assert_eq!(result, vec!["echo", "single", "double", "unquoted"]);
    }

    #[test]
    fn test_tokenize_adjacent_quoted_strings() {
        let result = tokenize_input(r#"echo "hello"'world'"#);
        assert_eq!(result, vec!["echo", "helloworld"]);
    }

    #[test]
    fn test_tokenize_redirect_operators() {
        let result = tokenize_input("echo hello > file.txt");
        assert_eq!(result, vec!["echo", "hello", ">", "file.txt"]);
    }

    #[test]
    fn test_tokenize_append_redirect() {
        let result = tokenize_input("echo hello >> file.txt");
        assert_eq!(result, vec!["echo", "hello", ">>", "file.txt"]);
    }

    #[test]
    fn test_tokenize_stderr_redirect() {
        let result = tokenize_input("cat file 2> error.txt");
        assert_eq!(result, vec!["cat", "file", "2>", "error.txt"]);
    }

    #[test]
    fn test_tokenize_complex_command() {
        let result = tokenize_input(r#"grep "pattern with spaces" file.txt | sort -n"#);
        assert_eq!(
            result,
            vec!["grep", "pattern with spaces", "file.txt", "|", "sort", "-n"]
        );
    }

    #[test]
    fn test_tokenize_empty_quotes() {
        let result = tokenize_input(r#"echo """#);
        assert_eq!(result, vec!["echo"]);
    }

    #[test]
    fn test_tokenize_quote_inside_different_quote() {
        let result = tokenize_input(r#"echo "it's working""#);
        assert_eq!(result, vec!["echo", "it's working"]);
    }

    #[test]
    fn test_tokenize_escaped_pipe() {
        let result = tokenize_input(r"echo hello\|world");
        assert_eq!(result, vec!["echo", "hello|world"]);
    }
}

#[cfg(test)]
mod parse_command_tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let commands = parse_input("echo hello");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello"]);
        assert_eq!(commands[0].stdout_redirect, None);
        assert_eq!(commands[0].stderr_redirect, None);
    }

    #[test]
    fn test_parse_command_with_multiple_args() {
        let commands = parse_input("echo hello world foo");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello", "world", "foo"]);
    }

    #[test]
    fn test_parse_stdout_redirect() {
        let commands = parse_input("echo hello > output.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello"]);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
        assert_eq!(commands[0].stdout_redirect_append, false);
    }

    #[test]
    fn test_parse_stdout_redirect_1() {
        let commands = parse_input("echo hello 1> output.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
        assert_eq!(commands[0].stdout_redirect_append, false);
    }

    #[test]
    fn test_parse_stdout_append_redirect() {
        let commands = parse_input("echo hello >> output.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello"]);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
        assert_eq!(commands[0].stdout_redirect_append, true);
    }

    #[test]
    fn test_parse_stdout_append_redirect_1() {
        let commands = parse_input("echo hello 1>> output.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
        assert_eq!(commands[0].stdout_redirect_append, true);
    }

    #[test]
    fn test_parse_stderr_redirect() {
        let commands = parse_input("cat file 2> error.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "cat");
        assert_eq!(commands[0].args, vec!["file"]);
        assert_eq!(
            commands[0].stderr_redirect,
            Some(PathBuf::from("error.txt"))
        );
        assert_eq!(commands[0].stderr_redirect_append, false);
    }

    #[test]
    fn test_parse_stderr_append_redirect() {
        let commands = parse_input("cat file 2>> error.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0].stderr_redirect,
            Some(PathBuf::from("error.txt"))
        );
        assert_eq!(commands[0].stderr_redirect_append, true);
    }

    #[test]
    fn test_parse_both_redirects() {
        let commands = parse_input("cat file > out.txt 2> err.txt");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].stdout_redirect, Some(PathBuf::from("out.txt")));
        assert_eq!(commands[0].stderr_redirect, Some(PathBuf::from("err.txt")));
    }

    #[test]
    fn test_parse_pipeline_two_commands() {
        let commands = parse_input("echo hello | cat");
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello"]);
        assert_eq!(commands[1].command, "cat");
        assert_eq!(commands[1].args.len(), 0);
    }

    #[test]
    fn test_parse_pipeline_three_commands() {
        let commands = parse_input("cat file | grep pattern | wc -l");
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0].command, "cat");
        assert_eq!(commands[1].command, "grep");
        assert_eq!(commands[1].args, vec!["pattern"]);
        assert_eq!(commands[2].command, "wc");
        assert_eq!(commands[2].args, vec!["-l"]);
    }

    #[test]
    fn test_parse_pipeline_with_redirect() {
        let commands = parse_input("cat file | grep pattern > output.txt");
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].command, "cat");
        assert_eq!(commands[1].command, "grep");
        assert_eq!(
            commands[1].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
    }

    #[test]
    fn test_parse_empty_input() {
        let commands = parse_input("");
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let commands = parse_input("   \t  ");
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_parse_command_with_quoted_args() {
        let commands = parse_input(r#"echo "hello world""#);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].command, "echo");
        assert_eq!(commands[0].args, vec!["hello world"]);
    }

    #[test]
    fn test_parse_redirect_with_quoted_filename() {
        let commands = parse_input(r#"echo hello > "output file.txt""#);
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output file.txt"))
        );
    }

    #[test]
    fn test_parse_args_after_redirect() {
        let commands = parse_input("echo hello > output.txt world");
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].args, vec!["hello", "world"]);
        assert_eq!(
            commands[0].stdout_redirect,
            Some(PathBuf::from("output.txt"))
        );
    }
}
