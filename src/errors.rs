use camino::Utf8PathBuf;
use std::fmt::{self, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    /// Failed to walk directory.
    DirWalk(walkdir::Error),
    /// Failed to open rust file.
    OpenRustFile { path: Utf8PathBuf, err: io::Error },
    /// Failed to render.
    Render { err: io::Error },
    /// Failed to specified output file.
    OpenOutputFile { path: Utf8PathBuf, err: io::Error },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            // TODO: more specific diagnose error by using error api.(path, io_error,...)
            DirWalk(err) => write!(f, "dirwalk: {err}"),
            OpenRustFile { path, err } => write!(f, "path: {path} {err}"),
            Render { err } => write!(f, "render: {err}"),
            OpenOutputFile { path, err } => write!(f, "open output file. path: {path} {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::DirWalk(err)
    }
}

impl Error {
    pub fn open_rust_file(path: impl Into<Utf8PathBuf>, err: io::Error) -> Self {
        Error::OpenRustFile {
            path: path.into(),
            err,
        }
    }

    pub fn open_output_file(path: impl Into<Utf8PathBuf>, err: io::Error) -> Self {
        Error::OpenOutputFile {
            path: path.into(),
            err,
        }
    }

    pub fn render(err: io::Error) -> Self {
        Error::Render { err }
    }
}
