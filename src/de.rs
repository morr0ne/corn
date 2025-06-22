use std::borrow::Cow;

use indexmap::IndexMap;
use serde::de::{self, IntoDeserializer};

use crate::{
    ast::{Entry, EntryOrSpread, Inputs, PairOrSpread, Root},
    lexer::{Lexer, StringPart},
    parser::RootParser,
    BorrowedValue, Error, Result,
};

#[derive(Clone)]
pub struct Deserializer<'de> {
    value: BorrowedValue<'de>,
}

pub fn parse(input: &str) -> Result<BorrowedValue> {
    Deserializer::parse(input)
}

impl<'de> Deserializer<'de> {
    pub fn parse(input: &str) -> Result<BorrowedValue> {
        let mut lexer = Lexer::new(input);
        let parser = RootParser::new();
        let Root { inputs, object } = parser.parse(input, &mut lexer).expect("Failed to parse"); // FIXME: handler errors

        Self::resolve_entry(&Entry::Object(object), &inputs)
    }

    pub fn from_str(input: &'de str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let parser = RootParser::new();
        let Root { inputs, object } = parser.parse(input, &mut lexer).expect("Failed to parse"); // FIXME: handler errors

        Ok(Self {
            value: Self::resolve_entry(&Entry::Object(object), &inputs)?,
        })
    }

    fn with_value(value: BorrowedValue<'de>) -> Self {
        Self { value }
    }

    fn resolve_entry<'input>(
        entry: &Entry<'input>,
        inputs: &Inputs<'input>,
    ) -> Result<BorrowedValue<'input>> {
        match entry {
            Entry::String(parts) => {
                if parts.is_empty() {
                    return Ok(BorrowedValue::String(Cow::Borrowed("")));
                }

                let mut base = String::new();

                for part in parts {
                    match part {
                        StringPart::Literal(lit) => base.push_str(lit),
                        StringPart::Input(input) => {
                            let input = Self::resolve_input(input, inputs)?;

                            match input {
                                BorrowedValue::String(string) => base.push_str(&string),
                                _ => panic!("Only strings can be interpolated into string"), // FIXME: Custom error
                            }
                        }
                    }
                }

                Ok(BorrowedValue::String(Cow::Owned(base)))
            }
            Entry::Integer(integer) => Ok(BorrowedValue::Integer(*integer)),
            Entry::Float(float) => Ok(BorrowedValue::Float(*float)),
            Entry::Boolean(boolean) => Ok(BorrowedValue::Boolean(*boolean)),
            Entry::Object(obj) => {
                let mut resolved_object = IndexMap::new();

                for pair_or_spread in &obj.pairs {
                    match pair_or_spread {
                        PairOrSpread::Pair(key, value) => {
                            Self::insert_at_path(
                                &mut resolved_object,
                                &key.segments,
                                Self::resolve_entry(value, inputs)?,
                            )?;
                        }
                        PairOrSpread::Spread(name) => {
                            if let Some(spread_entry) = inputs.get(name) {
                                match Self::resolve_entry(spread_entry, inputs)? {
                                    BorrowedValue::Object(spread_obj) => {
                                        for (k, v) in spread_obj {
                                            resolved_object.insert(k, v);
                                        }
                                    }
                                    _ => {
                                        return Err(Error::DeserializationError(format!(
                                            "Cannot spread non-object type: {}",
                                            name
                                        )))
                                    }
                                }
                            } else {
                                return Err(Error::DeserializationError(format!(
                                    "Undefined input for spread: {}",
                                    name
                                )));
                            }
                        }
                    }
                }

                Ok(BorrowedValue::Object(resolved_object))
            }
            Entry::Array(items) => {
                let mut resolved_array = Vec::with_capacity(items.len()); // We need at least the same amount of items

                for entry in items {
                    match entry {
                        EntryOrSpread::Entry(entry) => {
                            resolved_array.push(Self::resolve_entry(entry, inputs)?)
                        }
                        EntryOrSpread::Spread(spread) => match Self::resolve_input(spread, inputs)?
                        {
                            BorrowedValue::Array(array) => {
                                resolved_array.extend(array);
                            }
                            _ => panic!("Only arrays support being spreaded"), // FIXME: return an error
                        },
                    }
                }

                Ok(BorrowedValue::Array(resolved_array))
            }
            Entry::Null => Ok(BorrowedValue::Null),
            Entry::Input(input) => Self::resolve_input(input, inputs),
        }
    }

    fn insert_at_path<'input>(
        obj: &mut IndexMap<&'input str, BorrowedValue<'input>>,
        path: &[&'input str],
        value: BorrowedValue<'input>,
    ) -> Result<(), Error> {
        if path.is_empty() {
            return Err(Error::DeserializationError("Empty path".to_string()));
        }

        if path.len() == 1 {
            obj.insert(path[0], value);
            return Ok(());
        }

        let (first, rest) = path.split_first().unwrap();
        let entry = obj
            .entry(first)
            .or_insert_with(|| BorrowedValue::Object(indexmap::IndexMap::new()));

        match entry {
            BorrowedValue::Object(nested_obj) => {
                Self::insert_at_path(nested_obj, rest, value)?;
            }
            _ => {
                return Err(Error::DeserializationError(format!(
                    "Cannot index into non-object at key: {}",
                    first
                )));
            }
        }

        Ok(())
    }

    fn resolve_input<'input>(
        input: &str,
        inputs: &Inputs<'input>,
    ) -> Result<BorrowedValue<'input>> {
        if let Some(env) = input.strip_prefix("env_") {
            if let Ok(env) = std::env::var(env) {
                return Ok(BorrowedValue::String(Cow::Owned(env)));
            }
        }

        if let Some(entry) = inputs.get(input) {
            return Self::resolve_entry(entry, inputs);
        }

        panic!("No input found") // FIXME: return an error
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;

    T::deserialize(&mut deserializer)
}

macro_rules! deserialize_number {
    ($method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                BorrowedValue::Integer(integer) => integer.deserialize_any(visitor),
                ref value => Err(value.invalid_type("Integer")),
            }
        }
    };
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => visitor.visit_str(string),
            BorrowedValue::Integer(integer) => integer.deserialize_any(visitor),
            BorrowedValue::Float(float) => visitor.visit_f64(float),
            BorrowedValue::Boolean(boolean) => visitor.visit_bool(boolean),
            BorrowedValue::Null => visitor.visit_unit(),
            BorrowedValue::Array(ref mut items) => {
                let mut seq = Vec::new();
                std::mem::swap(items, &mut seq);

                visitor.visit_seq(SeqAccess::new(seq))
            }
            BorrowedValue::Object(ref mut object) => {
                let mut map = IndexMap::new();
                std::mem::swap(object, &mut map);

                visitor.visit_map(MapAccess::new(map))
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Boolean(boolean) => visitor.visit_bool(boolean),
            ref value => Err(value.invalid_type("Boolean")),
        }
    }

    deserialize_number!(deserialize_i8);
    deserialize_number!(deserialize_i16);
    deserialize_number!(deserialize_i32);
    deserialize_number!(deserialize_i64);
    deserialize_number!(deserialize_u8);
    deserialize_number!(deserialize_u16);
    deserialize_number!(deserialize_u32);
    deserialize_number!(deserialize_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Float(float) => visitor.visit_f64(float),
            ref value => Err(value.invalid_type("Float")),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => visitor.visit_str(string),
            ref value => Err(value.invalid_type("String")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => visitor.visit_bytes(string.as_bytes()),
            ref value => Err(value.invalid_type("Byte String")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Null => visitor.visit_unit(),
            ref value => Err(value.invalid_type("Null")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Array(ref mut items) => {
                let mut seq = Vec::new();
                std::mem::swap(items, &mut seq);

                visitor.visit_seq(SeqAccess::new(seq))
            }
            ref value => Err(value.invalid_type("Array")),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::Object(ref mut object) => {
                let mut map = IndexMap::new();
                std::mem::swap(object, &mut map);

                visitor.visit_map(MapAccess::new(map))
            }
            ref value => Err(value.invalid_type("Object")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            BorrowedValue::String(ref string) => {
                visitor.visit_enum(string.as_ref().into_deserializer())
            }
            ref value => Err(value.invalid_type("String or Object")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'de> {
    items: std::vec::IntoIter<BorrowedValue<'de>>,
}

impl<'de> SeqAccess<'de> {
    pub fn new(items: Vec<BorrowedValue<'de>>) -> Self {
        Self {
            items: items.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.items.next() {
            Some(item) => {
                let mut deserializer = Deserializer::with_value(item);
                seed.deserialize(&mut deserializer).map(Some)
            }
            None => Ok(None),
        }
    }
}

struct MapAccess<'de> {
    items: indexmap::map::IntoIter<&'de str, BorrowedValue<'de>>,
    current_value: Option<BorrowedValue<'de>>,
}

impl<'de> MapAccess<'de> {
    fn new(items: IndexMap<&'de str, BorrowedValue<'de>>) -> Self {
        Self {
            items: items.into_iter(),
            current_value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.items.next() {
            Some((key, value)) => {
                self.current_value = Some(value);
                let mut key_deserializer =
                    Deserializer::with_value(BorrowedValue::String(Cow::Borrowed(key)));
                seed.deserialize(&mut key_deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.current_value.take() {
            Some(value) => {
                let mut deserializer = Deserializer::with_value(value);
                seed.deserialize(&mut deserializer)
            }
            None => Err(Error::DeserializationError(
                "No value available".to_string(),
            )),
        }
    }
}
