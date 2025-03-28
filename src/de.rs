use std::collections::HashMap;

use serde::de;

use crate::{Error, Value};

#[derive(Clone)]
pub struct Deserializer<'de> {
    bytes: &'de [u8],
    index: usize,
    variables: HashMap<String, Value>,
}

impl Deserializer<'_> {
    pub fn from_str(input: &str) -> Self {
        todo!()
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);

    T::deserialize(&mut deserializer)
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

// impl Entry {
//     pub const fn as_type(&self) -> &'static str {
//         match self {
//             Self::String(_) => todo!(),
//             // Self::InterpolatedString(string_parts) => todo!(),
//             Self::Integer(_) => todo!(),
//             Self::Float(_) => todo!(),
//             Self::Boolean(_) => todo!(),
//             Self::Object(object_entry) => todo!(),
//             Self::Array(array_entry) => todo!(),
//             Self::Input(_) => todo!(),
//             Self::Null => todo!(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum ObjectEntry {
//     Flat(IndexMap<String, Entry>),
//     WithSpreads(Vec<ObjectPart>),
// }

// #[derive(Debug, Clone)]
// pub enum ObjectPart {
//     Pair(String, Entry),
//     Spread(String),
// }

// #[derive(Debug, Clone)]
// pub enum ArrayEntry {
//     Flat(Vec<Entry>),
//     WithSpreads(Vec<ArrayPart>),
// }

// /// Part of an array with spreads
// #[derive(Debug, Clone)]
// pub enum ArrayPart {
//     Entry(Entry),
//     Spread(String),
// }

// /// Helpers for the parser
// #[derive(Debug, Clone)]
// pub(crate) enum SpreadOr<T> {
//     Spread(String),
//     Other(T),
// }

// pub(crate) fn pairs_to_object(pairs: Vec<SpreadOr<(String, Entry)>>) -> ObjectEntry {
//     let has_spreads = pairs.iter().any(|p| matches!(p, SpreadOr::Spread(_)));

//     if has_spreads {
//         let parts: Vec<ObjectPart> = pairs
//             .into_iter()
//             .map(|p| match p {
//                 SpreadOr::Other((k, v)) => ObjectPart::Pair(k, v),
//                 SpreadOr::Spread(name) => ObjectPart::Spread(name),
//             })
//             .collect();
//         ObjectEntry::WithSpreads(parts)
//     } else {
//         let map: IndexMap<String, Entry> = pairs
//             .into_iter()
//             .filter_map(|p| match p {
//                 SpreadOr::Other((k, v)) => Some((k, v)),
//                 _ => None, // This should never happen if has_spreads is false
//             })
//             .collect();
//         ObjectEntry::Flat(map)
//     }
// }

// pub(crate) fn entries_to_array(entries: Vec<SpreadOr<Entry>>) -> ArrayEntry {
//     let has_spreads = entries.iter().any(|e| matches!(e, SpreadOr::Spread(_)));

//     if has_spreads {
//         let parts: Vec<ArrayPart> = entries
//             .into_iter()
//             .map(|e| match e {
//                 SpreadOr::Other(v) => ArrayPart::Entry(v),
//                 SpreadOr::Spread(name) => ArrayPart::Spread(name),
//             })
//             .collect();
//         ArrayEntry::WithSpreads(parts)
//     } else {
//         let values: Vec<Entry> = entries
//             .into_iter()
//             .filter_map(|e| match e {
//                 SpreadOr::Other(v) => Some(v),
//                 _ => None, // This should never happen if has_spreads is false
//             })
//             .collect();
//         ArrayEntry::Flat(values)
//     }
// }

// // pub(crate) fn create_nested_entry(keys: Vec<String>, value: Entry) -> Entry {
// //     let mut current = value;

// //     for key in keys.into_iter().rev() {
// //         current = Entry::Object(ObjectEntry::Flat(indexmap! {key => current}));
// //     }

// //     current
// // }
