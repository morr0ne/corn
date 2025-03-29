use indexmap::IndexMap;

mod de;
mod ser;

pub type Object = IndexMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64), // FIXME: Use a custom number wrapper to handle both signed and unsigned integers
    Float(f64),
    Boolean(bool),
    Object(Object),
    Array(Vec<Value>),
    Null,
}
