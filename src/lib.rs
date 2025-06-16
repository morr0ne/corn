// use indexmap::IndexMap;
// use serde::Serialize;
// use std::borrow::Cow;
// use std::collections::HashMap;
// use std::fmt::{Display, Formatter};

// pub use crate::de::{from_slice, from_str};
// pub use crate::parser::{parse, Rule};

// pub mod error;
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


mod value;

pub mod ast;
pub mod lexer;

use std::fmt::Display;

pub use value::{Integer, Object, Value};

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

pub fn parse(file: &str) -> Result<Value, Error> {
    todo!()
}
