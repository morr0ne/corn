// use indexmap::IndexMap;
// use serde::Serialize;
// use std::borrow::Cow;
// use std::collections::HashMap;
// use std::fmt::{Display, Formatter};

// pub use crate::de::{from_slice, from_str};
// pub use crate::parser::{parse, Rule};

// mod parser;

// mod de;
// #[cfg(any(
//     feature = "lua51",
//     feature = "lua52",
//     feature = "lua53",
//     feature = "lua54",
//     feature = "luajit",
//     feature = "luajit52"
// ))]
// mod lua;
// #[cfg(feature = "wasm")]
// mod wasm;

lalrpop_util::lalrpop_mod!(pub parser, "/corn.rs");

mod error;
mod value;

pub use error::{Error, Result};

pub mod ast;
pub mod lexer;

pub use value::{Integer, Object, Value};

pub fn parse(_file: &str) -> Result<Value, Error> {
    todo!()
}
