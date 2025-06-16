use std::collections::HashMap;

use crate::Integer;

/// Store for input declarations
pub type Inputs = HashMap<String, Entry>;

/// Top level ast object
#[derive(Debug, Clone)]
pub struct Root {
    /// Raw inputs
    pub inputs: Inputs,
    /// Top level object, values aren't interpolated
    pub object: Object,
}

/// Represents an object in the AST
#[derive(Debug, Clone)]
pub struct Object {
    // /// The pairs or spread operations in the object
    pub pairs: Vec<PairOrSpread>,
}

/// Either a key-value pair or a spread operation
#[derive(Debug, Clone)]
pub enum PairOrSpread {
    /// A key-value pair in an object
    Pair(ChainedKey, Entry),
    /// A spread operation in an object
    Spread(String),
}

/// An entry can be of various types as defined in the spec
#[derive(Debug, Clone)]
pub enum Entry {
    /// String literal
    String(String),
    /// Integer value
    Integer(Integer),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Nested object
    Object(Object),
    /// Array of entries
    Array(Vec<Entry>),
    /// Null value
    Null,
    /// Reference to an input variable
    Input(String),
    /// Array spread operation
    Spread(String),
}

/// Represents a chained key like "foo.bar.baz"
#[derive(Debug, Clone)]
pub struct ChainedKey {
    /// The segments of the key path
    pub segments: Vec<String>,
}

impl ChainedKey {
    pub fn new(key: String) -> Self {
        Self {
            segments: vec![key],
        }
    }
}
