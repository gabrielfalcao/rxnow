use iocore::Exception;
use std::env::VarError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    FileSystemError(String),
    WalkDirError(walkdir::Error),
    IOException(Exception),
    StringError(std::string::String),
    IOError(std::io::Error),
    InvalidUtf8(FromUtf8Error),
    RegexError(regex::Error),
    VarError(VarError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FileSystemError(e) => write!(f, "FileSystemError: {}", e),
            Error::WalkDirError(e) => write!(f, "WalkDirError: {}", e),
            Error::IOError(e) => write!(f, "I/O Error: {}", e),
            Error::StringError(e) => write!(f, "StringError: {}", e),
            Error::IOException(e) => write!(f, "{}", e),
            Error::InvalidUtf8(s) => write!(f, "InvalidUtf8: {}", s),
            Error::VarError(s) => write!(f, "VarError: {}", s),
            Error::RegexError(ror) => match ror {
                regex::Error::Syntax(xatnys) => write!(f, "{}", xatnys),
                s => write!(f, "{}", s),
            },
        }
    }
}

impl std::error::Error for Error {}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::RegexError(e)
    }
}
impl From<walkdir::Error> for Error {
    fn from(e: walkdir::Error) -> Self {
        Error::WalkDirError(e)
    }
}

impl From<std::string::String> for Error {
    fn from(e: std::string::String) -> Self {
        Error::StringError(e)
    }
}

impl From<Exception> for Error {
    fn from(e: Exception) -> Self {
        Error::IOException(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::InvalidUtf8(e)
    }
}
impl From<VarError> for Error {
    fn from(e: VarError) -> Self {
        Error::VarError(e)
    }
}
