mod de;
#[cfg(any(
    feature = "lua51",
    feature = "lua52",
    feature = "lua53",
    feature = "lua54",
    feature = "luajit",
    feature = "luajit52"
))]
mod lua;

mod error;
pub(crate) mod value;

pub mod ast;
pub mod lexer;
lalrpop_util::lalrpop_mod!(pub parser, "/corn.rs");

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use value::{Integer, Object, Value};
