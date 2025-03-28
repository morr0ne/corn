use std::fmt::Display;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),

    // #[error("failed to resolve referenced input `{0}`")]
    // InputResolveError(String),

    // #[error("attempted to use dot-notation on non-object value at `{0}`")]
    // InvalidPathError(String),

    // #[error("attempted to spread a type that differs from its containing type at `{0}`")]
    // InvalidSpreadError(String),

    // #[error("attempted to interpolate a non-string type into a string at `{0}`")]
    // InvalidInterpolationError(String),

    // #[error("failed to deserialize input: {0}")]
    DeserializationError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::DeserializationError(s) => s.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
