use std::fmt;
use std::path::PathBuf;
use std::convert::From;
use std::ffi::OsString;
use std::io::Error as IoError;

#[macro_use] use failure::{Context, Fail, Backtrace};
use walkdir::Error as WalkDirError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "File {:?} is too large to handle", path)]
    FileTooLarge { path: PathBuf },

    #[fail(display = "Path {:?} does not contain a file", path)]
    NotAFile { path: PathBuf },

    #[fail(display = "File {:?} is system-protected", path)]
    IsSystemFile { path: PathBuf },

    #[fail(display = "File {:?} is read-only", path)]
    IsReadOnly { path: PathBuf },

    #[fail(display = "File {:?} has malformed input", path)]
    MalformedInput { path: PathBuf },

    #[fail(display = "Output buffer was exhausted when decoding file {:?}", path)]
    OutputBufferExhausted { path: PathBuf },

    #[fail(display = "Unsupported encoding {:?}", name)]
    UnsupportedEncoding { name: String },

    #[fail(display = "Unsupported MIME type {:?}", mime_type)]
    UnsupportedMimeType { mime_type: String },

    #[fail(display = "Error converting from OS String {:?}", original)]
    FfiOsString { original: OsString },

    #[fail(display = "Error traversing directory {:?}", 0)]
    WalkDir(#[cause] WalkDirError),

    #[fail(display = "I/O Error {:?}", 0)]
    Io(#[cause] IoError)
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind_ref(&self) -> &ErrorKind {
        self.inner.get_context()
    }

    pub fn file_too_large(path: &PathBuf) -> Error {
        Error::from(ErrorKind::FileTooLarge { path: path.to_owned() })
    }

    pub fn not_a_file(path: &PathBuf) -> Error {
        Error::from(ErrorKind::NotAFile { path: path.to_owned() })
    }

    pub fn is_system_file(path: &PathBuf) -> Error {
        Error::from(ErrorKind::IsSystemFile { path: path.to_owned() })
    }

    pub fn is_read_only(path: &PathBuf) -> Error {
        Error::from(ErrorKind::IsReadOnly { path: path.to_owned() })
    }

    pub fn malformed_input(path: &PathBuf) -> Error {
        Error::from(ErrorKind::MalformedInput { path: path.to_owned() })
    }

    pub fn output_buffer_exhausted(path: &PathBuf) -> Error {
        Error::from(ErrorKind::OutputBufferExhausted { path: path.to_owned() })
    }

    pub fn unsupported_encoding(name: &str) -> Error {
        Error::from(ErrorKind::UnsupportedEncoding { name: name.to_owned() })
    }

    pub fn unsupported_mime_type(mime_type: &str) -> Error {
        Error::from(ErrorKind::UnsupportedMimeType { mime_type: mime_type.to_owned() })
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<IoError> for Error {
    fn from(cause: IoError) -> Error {
        Error { inner: Context::new(ErrorKind::Io(cause)) }
    }
}
