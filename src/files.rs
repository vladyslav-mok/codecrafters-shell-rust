use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::error::{ShellError, ShellResult};

pub fn open_file(path: &Path, append: bool) -> ShellResult<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(path)
        .map_err(|e| ShellError::FileOpen {
            path: path.display().to_string(),
            source: e,
        })
}
