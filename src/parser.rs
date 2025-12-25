const SPECIAL_CHARS: &[&str] = &["\"", "\\"];

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenizerState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
    Escaped,
    EscapedInDoubleQuote,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RedirectType {
    StdoutTruncate,
    StdoutAppend,
    StderrTruncate,
    StderrAppend,
}

impl RedirectType {
    fn from_operator(op: &str) -> Option<Self> {
        match op {
            ">" | "1>" => Some(RedirectType::StdoutTruncate),
            ">>" | "1>>" => Some(RedirectType::StdoutAppend),
            "2>" => Some(RedirectType::StderrTruncate),
            "2>>" => Some(RedirectType::StderrAppend),
            _ => None,
        }
    }
}

use std::path::PathBuf;

#[derive(Debug)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,

    pub stdout_redirect: Option<PathBuf>,
    pub stderr_redirect: Option<PathBuf>,

    pub stdout_redirect_append: bool,
    pub stderr_redirect_append: bool,
}

pub fn parse_input(input: &str) -> Vec<ParsedCommand> {
    let tokens = tokenize_input(input);
    let mut commands: Vec<ParsedCommand> = Vec::new();

    for token in tokens.split(|t| t == "|") {
        if token.is_empty() {
            continue;
        }

        if let Some(parsed_command) = parse_command_line(token.to_vec()) {
            commands.push(parsed_command);
        }
    }

    commands
}

pub fn parse_command_line(tokens: Vec<String>) -> Option<ParsedCommand> {
    let command = tokens[0].clone();
    let mut args = Vec::new();
    let mut stdout_redirect = None;
    let mut stderr_redirect = None;

    let mut stdout_redirect_append = false;
    let mut stderr_redirect_append = false;

    let mut iter = tokens.iter().skip(1).peekable();

    while let Some(token) = iter.next() {
        if let Some(redirect_type) = RedirectType::from_operator(token.as_str()) {
            if let Some(path) = iter.next() {
                match redirect_type {
                    RedirectType::StdoutTruncate => {
                        stdout_redirect = Some(PathBuf::from(path));
                        stdout_redirect_append = false;
                    }
                    RedirectType::StdoutAppend => {
                        stdout_redirect = Some(PathBuf::from(path));
                        stdout_redirect_append = true;
                    }
                    RedirectType::StderrTruncate => {
                        stderr_redirect = Some(PathBuf::from(path));
                        stderr_redirect_append = false;
                    }
                    RedirectType::StderrAppend => {
                        stderr_redirect = Some(PathBuf::from(path));
                        stderr_redirect_append = true;
                    }
                }
            } else {
                eprintln!("Syntax error: expected file path after redirect");
            }
        } else {
            args.push(token.clone());
        }
    }

    Some(ParsedCommand {
        command,
        args,
        stdout_redirect,
        stderr_redirect,
        stdout_redirect_append,
        stderr_redirect_append,
    })
}

pub fn tokenize_input(input: &str) -> Vec<String> {
    let tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}

struct Tokenizer {
    chars: std::iter::Peekable<std::str::Chars<'static>>,
    state: TokenizerState,
    tokens: Vec<String>,
    current_token: String,
    _input: String,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        // Store input to control its lifetime
        let owned_input = input.to_string();
        // SAFETY: We're immediately consuming the chars iterator in tokenize()
        // and the input string is stored in the struct, so this is safe.
        let chars = unsafe {
            std::mem::transmute::<std::str::Chars<'_>, std::str::Chars<'static>>(
                owned_input.chars(),
            )
        }
        .peekable();

        Self {
            chars,
            state: TokenizerState::Normal,
            tokens: Vec::new(),
            current_token: String::new(),
            _input: owned_input,
        }
    }

    fn tokenize(mut self) -> Vec<String> {
        while let Some(c) = self.chars.next() {
            self.process_char(c);
        }

        self.finish_token();
        self.tokens
    }

    fn process_char(&mut self, c: char) {
        match self.state {
            TokenizerState::Normal => self.handle_normal(c),
            TokenizerState::InSingleQuote => self.handle_single_quote(c),
            TokenizerState::InDoubleQuote => self.handle_double_quote(c),
            TokenizerState::Escaped => self.handle_escaped(c),
            TokenizerState::EscapedInDoubleQuote => self.handle_escaped_in_double_quote(c),
        }
    }

    fn handle_normal(&mut self, c: char) {
        match c {
            '\\' => {
                self.state = TokenizerState::Escaped;
            }
            '\'' => {
                self.state = TokenizerState::InSingleQuote;
            }
            '"' => {
                self.state = TokenizerState::InDoubleQuote;
            }
            '|' => {
                self.finish_token();
                self.tokens.push("|".to_string());
            }
            c if c.is_whitespace() => {
                self.finish_token();
            }
            _ => {
                self.current_token.push(c);
            }
        }
    }

    fn handle_single_quote(&mut self, c: char) {
        match c {
            '\'' => {
                self.state = TokenizerState::Normal;
            }
            _ => {
                self.current_token.push(c);
            }
        }
    }

    fn handle_double_quote(&mut self, c: char) {
        match c {
            '\\' => {
                // Check if next char is a special char that should be escaped
                if let Some(&next_c) = self.chars.peek()
                    && SPECIAL_CHARS.contains(&next_c.to_string().as_str())
                {
                    self.state = TokenizerState::EscapedInDoubleQuote;
                    return;
                }
                // Not a special escape, treat backslash literally
                self.current_token.push(c);
            }
            '"' => {
                self.state = TokenizerState::Normal;
            }
            _ => {
                self.current_token.push(c);
            }
        }
    }

    fn handle_escaped(&mut self, c: char) {
        self.current_token.push(c);
        self.state = TokenizerState::Normal;
    }

    fn handle_escaped_in_double_quote(&mut self, c: char) {
        self.current_token.push(c);
        self.state = TokenizerState::InDoubleQuote;
    }

    fn finish_token(&mut self) {
        if !self.current_token.is_empty() {
            self.tokens.push(self.current_token.clone());
            self.current_token.clear();
        }
    }
}
