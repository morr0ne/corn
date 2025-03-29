use indexmap::IndexMap;

mod de;
mod integer;
mod ser;

pub use integer::Integer;

pub type Object = IndexMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(Integer),
    Float(f64),
    Boolean(bool),
    Object(Object),
    Array(Vec<Value>),
    Null,
}
