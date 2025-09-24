use alloc::string::{String, ToString};
use core::fmt::Display;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to resolve referenced input `{0}`")]
    InputResolveError(String),

    #[error("attempted to spread a type that differs from its containing type")]
    InvalidSpreadError,

    #[error("attempted to interpolate a non-string type into a string")]
    InvalidInterpolationError,

    #[error("failed to deserialize input: {0}")]
    DeserializationError(String),

    #[error("failed to parse input: {0}")]
    ParseError(String),
}

impl serde_core::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::DeserializationError(msg.to_string())
    }
}
