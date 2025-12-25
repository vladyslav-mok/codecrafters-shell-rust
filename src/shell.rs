use rustyline::completion::{Candidate, Completer};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

pub struct Shell {
    pub commands: Vec<String>,
}

impl Shell {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }
}

#[derive(Clone)]
pub struct CustomCandidate {
    display: String,
    replacement: String,
}

impl Candidate for CustomCandidate {
    fn display(&self) -> &str {
        &self.display
    }

    fn replacement(&self) -> &str {
        &self.replacement
    }
}

impl Completer for Shell {
    type Candidate = CustomCandidate;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context,
    ) -> Result<(usize, Vec<CustomCandidate>)> {
        let mut candidates: Vec<CustomCandidate> = Vec::new();

        if line.is_empty() {
            return Ok((0, candidates));
        }

        for command in &self.commands {
            if command.starts_with(line) {
                candidates.push(CustomCandidate {
                    display: command.clone(),
                    replacement: format!("{} ", command),
                });
            }
        }

        Ok((0, candidates))
    }
}

impl Helper for Shell {}

impl Hinter for Shell {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context) -> Option<String> {
        None
    }
}

impl Highlighter for Shell {}

impl Validator for Shell {
    fn validate(
        &self,
        _ctx: &mut rustyline::validate::ValidationContext,
    ) -> Result<rustyline::validate::ValidationResult> {
        Ok(rustyline::validate::ValidationResult::Valid(None))
    }
}
