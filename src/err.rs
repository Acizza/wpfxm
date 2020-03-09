use snafu::{Backtrace, ErrorCompat, GenerateBacktrace, Snafu};
use std::io;
use std::path;
use std::result;
use std::sync::mpsc;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("io error: {}", source))]
    IO {
        source: io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("mpsc channel receive error: {}", source))]
    MPSCRecv {
        source: mpsc::RecvError,
        backtrace: Backtrace,
    },

    #[snafu(display("file io error at {}: {}", path.display(), source))]
    FileIO {
        path: path::PathBuf,
        source: io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("dir entry error at {}: {}", dir.display(), source))]
    EntryIO {
        dir: path::PathBuf,
        source: io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("toml decode error at {}: {}", path.display(), source))]
    TomlDecode {
        path: path::PathBuf,
        source: toml::de::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("toml encode error at {}: {}", path.display(), source))]
    TomlEncode {
        path: path::PathBuf,
        source: toml::ser::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("{} is not a valid Wine prefix", path.display()))]
    NotAPrefix { path: path::PathBuf },

    #[snafu(display("failed to detect prefix arch at {}", path.display()))]
    NoArchDetected { path: path::PathBuf },
}

impl Error {
    pub fn is_file_nonexistant(&self) -> bool {
        match self {
            Error::FileIO { source, .. } => source.kind() == io::ErrorKind::NotFound,
            _ => false,
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Error {
        Error::IO {
            source,
            backtrace: Backtrace::generate(),
        }
    }
}

pub fn display_error(err: Error) {
    eprintln!("{}", err);

    if let Some(backtrace) = err.backtrace() {
        eprintln!("backtrace:\n{}", backtrace);
    }
}
