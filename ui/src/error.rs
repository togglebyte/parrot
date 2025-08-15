use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Import(PathBuf),
    LoadValue(String),
    FilePath(PathBuf),
    Anathema(anathema::runtime::Error),
    Syntect(syntect::Error),
    InvalidTheme(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Import(path) => write!(f, "failed to load \"{}\"", path.to_str().unwrap_or("<path>")),
            Error::LoadValue(key) => write!(f, "\"{key}\" does not exist"),
            Error::FilePath(path_buf) => write!(f, "file does not exist: {}", path_buf.to_str().unwrap_or("<path>")),
            Error::Anathema(error) => write!(f, "{error}"),
            Error::Syntect(error) => write!(f, "{error}"),
            Error::InvalidTheme(theme) => write!(f, "no theme named \"{theme}\""),
        }
    }
}

impl std::error::Error for Error {
}

impl From<anathema::runtime::Error> for Error {
    fn from(e: anathema::runtime::Error) -> Self {
        Self::Anathema(e)
    }
}

impl From<syntect::Error> for Error {
    fn from(e: syntect::Error) -> Self {
        Self::Syntect(e)
    }
}
