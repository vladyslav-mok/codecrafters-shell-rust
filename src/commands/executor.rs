use std::fs::File;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command as ProcessCommand, Stdio};

use super::{CommandRegistry, ShellStatus};
use crate::error::ShellResult;
use crate::files::open_file;
use crate::parser::ParsedCommand;

enum PipeState {
    None,
    Process(Child),
    Buffer(Vec<u8>),
}

fn setup_file_redirect(
    redirect: &Option<std::path::PathBuf>,
    append: bool,
) -> ShellResult<Option<File>> {
    if let Some(path) = redirect {
        Ok(Some(open_file(path, append)?))
    } else {
        Ok(None)
    }
}

pub struct ShellExecutor<'a> {
    registry: &'a CommandRegistry,
}

impl<'a> ShellExecutor<'a> {
    pub fn new(registry: &'a CommandRegistry) -> Self {
        Self { registry }
    }

    pub fn run(&self, pipeline: &[ParsedCommand]) -> ShellResult<ShellStatus> {
        if pipeline.is_empty() {
            return Ok(ShellStatus::Continue);
        }

        let mut previous_output = PipeState::None;
        let mut iter = pipeline.iter().peekable();

        while let Some(cmd) = iter.next() {
            let is_last = iter.peek().is_none();

            let is_builtin = self.registry.get_builtin(&cmd.command).is_some();

            let (new_state, status) = if is_builtin {
                self.handle_builtin(cmd, &mut previous_output, is_last)?
            } else {
                self.handle_external(cmd, &mut previous_output, is_last)?
            };

            if let ShellStatus::Exit = status {
                return Ok(ShellStatus::Exit);
            }

            previous_output = new_state;
        }

        if let PipeState::Process(mut child) = previous_output {
            child.wait()?;
        }

        Ok(ShellStatus::Continue)
    }

    fn handle_builtin(
        &self,
        cmd: &ParsedCommand,
        _input: &mut PipeState,
        is_last: bool,
    ) -> ShellResult<(PipeState, ShellStatus)> {
        let builtin = self
            .registry
            .get_builtin(&cmd.command)
            .expect("handle_builtin called but builtin not found - this is a bug");

        let mut output_buffer = Vec::new();
        let mut writer: Box<dyn Write> = if let Some(file) =
            setup_file_redirect(&cmd.stdout_redirect, cmd.stdout_redirect_append)?
        {
            Box::new(file)
        } else if !is_last {
            Box::new(&mut output_buffer)
        } else {
            Box::new(io::stdout())
        };

        let _stderr_file = setup_file_redirect(&cmd.stderr_redirect, cmd.stderr_redirect_append)?;

        let result = builtin.execute(&cmd.args, self.registry, &mut *writer);

        drop(writer);

        match result {
            Ok(status) => {
                if !is_last && cmd.stdout_redirect.is_none() {
                    Ok((PipeState::Buffer(output_buffer), status))
                } else {
                    Ok((PipeState::None, status))
                }
            }
            Err(e) => {
                if let Some(mut file) = setup_file_redirect(&cmd.stderr_redirect, true)? {
                    writeln!(file, "{}", e)?;
                    Ok((PipeState::None, ShellStatus::Continue))
                } else {
                    Err(e)
                }
            }
        }
    }

    fn handle_external(
        &self,
        cmd: &ParsedCommand,
        input: &mut PipeState,
        is_last: bool,
    ) -> ShellResult<(PipeState, ShellStatus)> {
        let Some(full_path) = self.registry.get_executable_path(&cmd.command) else {
            return Err(crate::error::ShellError::CommandNotFound(
                cmd.command.clone(),
            ));
        };

        let stdin = match input {
            PipeState::Process(child) => {
                if let Some(out) = child.stdout.take() {
                    Stdio::from(out)
                } else {
                    Stdio::null()
                }
            }
            PipeState::Buffer(_) => Stdio::piped(),
            PipeState::None => Stdio::inherit(),
        };

        let (stdout, creates_pipe) = if let Some(file) =
            setup_file_redirect(&cmd.stdout_redirect, cmd.stdout_redirect_append)?
        {
            (Stdio::from(file), false)
        } else if !is_last {
            (Stdio::piped(), true)
        } else {
            (Stdio::inherit(), false)
        };

        let stderr = if let Some(file) =
            setup_file_redirect(&cmd.stderr_redirect, cmd.stderr_redirect_append)?
        {
            Stdio::from(file)
        } else {
            Stdio::inherit()
        };

        let mut command_builder = ProcessCommand::new(&full_path);

        command_builder
            .arg0(&cmd.command)
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr);

        let mut child =
            command_builder
                .spawn()
                .map_err(|e| crate::error::ShellError::ProcessStart {
                    command: cmd.command.clone(),
                    source: e,
                })?;

        if let PipeState::Buffer(data) = input
            && let Some(mut stdin) = child.stdin.take()
        {
            stdin.write_all(data)?;
        }

        if creates_pipe {
            Ok((PipeState::Process(child), ShellStatus::Continue))
        } else {
            child.wait()?;
            Ok((PipeState::None, ShellStatus::Continue))
        }
    }
}
