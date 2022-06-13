#[macro_use]
#[cfg(target_os = "macos")]
extern crate objc;

#[cfg(target_os = "linux")]
mod linux;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use linux::read_clipboard;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::read_clipboard;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::read_clipboard;

/// Read the system-wide clipboard. Returns a list of one or more absolute file paths or an error.
pub fn read() -> Result<Vec<PathBuf>, Error> {
    read_clipboard().map(|strs| strs.into_iter().map(PathBuf::from).collect())
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoFiles,
    SystemError(String),
}
