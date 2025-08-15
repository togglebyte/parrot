use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Import(PathBuf),
    Load(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Import(path) => write!(f, "failed to load \"{}\"", path.to_str().unwrap_or("<path>")),
            Error::Load(key) => write!(f, "\"{key}\" does not exist"),
        }
    }
}

impl std::error::Error for Error {}
