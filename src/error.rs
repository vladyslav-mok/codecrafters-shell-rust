use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShellError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("{0}: command not found")]
    CommandNotFound(String),

    #[error("cd: {path}: No such file or directory")]
    DirectoryNotFound { path: String },

    #[error("{0}: not found")]
    TypeNotFound(String),

    #[error("history: {flag}: argument required")]
    HistoryArgRequired { flag: String },

    #[error("history: {arg}: numeric argument required")]
    HistoryInvalidArg { arg: String },

    #[error("Failed to open {path}: {source}")]
    FileOpen {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("Failed to start {command}: {source}")]
    ProcessStart {
        command: String,
        #[source]
        source: io::Error,
    },
}

pub type ShellResult<T> = Result<T, ShellError>;
