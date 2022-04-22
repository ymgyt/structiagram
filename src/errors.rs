use camino::Utf8PathBuf;
use std::fmt::{self, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    /// Failed to walk directory.
    DirWalkError(walkdir::Error),
    /// Failed to open rust file.
    OpenRustFile { path: Utf8PathBuf, err: io::Error },
    /// Failed to render
    RenderError { err: io::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            // TODO: more specific diagnose error by using error api.(path, io_error,...)
            DirWalkError(err) => write!(f, "dirwalk: {err}"),
            OpenRustFile { path, err } => write!(f, "path: {path} {err}"),
            RenderError { err } => write!(f, "render: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::DirWalkError(err)
    }
}

impl Error {
    pub fn open_rust_file(path: impl Into<Utf8PathBuf>, err: io::Error) -> Self {
        Error::OpenRustFile {
            path: path.into(),
            err,
        }
    }

    pub fn render(err: io::Error) -> Self {
        Error::RenderError { err }
    }
}
