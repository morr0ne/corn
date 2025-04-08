use std::collections::HashMap;

use serde::de;

use crate::{Error, Result, Value};

/// A structure that deserializes Corn into Rust values.
#[derive(Clone)]
pub struct Deserializer<'de> {
    bytes: &'de [u8],
    index: usize,
    variables: HashMap<String, Value>,
}

impl<'de> Deserializer<'de> {
    /// Refer to the `Deserializer::from_str` method for more info.
    pub fn from_str(input: &'de str) -> Result<Self> {
        let mut de = Self {
            bytes: input.as_bytes(),
            index: 0,
            variables: HashMap::new(),
        };

        de.parse_let_block()?;

        Ok(de)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn next(&mut self) -> Result<Option<u8>> {
        let byte = self.peek()?;

        self.advance();

        Ok(byte)
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        Ok(self.bytes.get(self.index).copied())
    }

    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match self.peek()? {
                Some(byte) if byte.is_ascii_whitespace() => {
                    self.advance();
                }
                other => return Ok(other),
            }
        }
    }

    fn whitespace_or_eof(&mut self) -> Result<u8> {
        match self.parse_whitespace() {
            Ok(Some(byte)) => Ok(byte),
            Ok(None) => Err(Error::Eof),
            Err(err) => Err(err),
        }
    }

    fn parse_key(&mut self) -> Result<String> {
        todo!()
    }

    fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
        for expected in ident {
            match self.next()? {
                None => {
                    return Err(Error::Eof);
                }
                Some(next) => {
                    if next != *expected {
                        return Err(Error::DeserializationError("No ident".to_string()));
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_integer(&mut self, negative: bool) -> Result<i64> {
        let next = self.whitespace_or_eof()?;

        match next {
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as i64;

                loop {
                    match self.next()? {
                        None => {
                            break Ok(if negative {
                                significand.wrapping_neg()
                            } else {
                                significand
                            })
                        }

                        Some(integer @ b'0'..=b'9') => {
                            let digit = (integer - b'0') as i64;

                            significand = significand * 10 + digit;
                        }

                        Some(token) => return Err(Error::unexpected_token("", token, self.index)),
                    }
                }
            }

            _ => todo!(),
        }
    }

    fn parse_let_block(&mut self) -> Result<()> {
        match self.whitespace_or_eof()? {
            b'{' => return Ok(()),
            b'l' => {
                self.parse_ident(b"let")?;

                match self.whitespace_or_eof()? {
                    b'{' => {
                        self.advance();
                        loop {
                            match self.whitespace_or_eof()? {
                                b'$' => {
                                    unimplemented!("key parsing")
                                }
                                b'}' => {
                                    self.advance();
                                    break;
                                }
                                token => {
                                    return Err(Error::unexpected_token(
                                        "input definition or }",
                                        token,
                                        self.index,
                                    ))
                                }
                            }
                        }

                        match self.whitespace_or_eof()? {
                            b'i' => {
                                self.parse_ident(b"in")?;
                            }
                            token => return Err(Error::unexpected_token("in", token, self.index)),
                        }
                    }
                    token => return Err(Error::unexpected_token("{", token, self.index)),
                }
            }
            token => return Err(Error::unexpected_token("one of: let, {", token, self.index)),
        }

        Ok(())
    }
}

/// Deserializes a Corn-formatted string into a Rust type.
///
/// # Example
///
/// ```
/// use corn::from_str;
///
/// #[derive(serde::Deserialize)]
/// struct Config {
///     name: String,
///     version: u32,
/// }
///
/// let corn_str = "{ name = \"My App\" version = 1 }";
/// let config: Config = from_str(corn_str).unwrap();
/// assert_eq!(config.name, "My App");
/// assert_eq!(config.version, 1);
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;

    T::deserialize(&mut deserializer)
}

// TODO: more specialized number parsing
// TODO: extract parsing logic from deserializer
impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // TODO: evaluate if forwarding method is correct
        match self.whitespace_or_eof()? {
            b'n' => {
                self.parse_ident(b"null")?;
                visitor.visit_unit()
            }
            b't' => {
                self.parse_ident(b"true")?;
                visitor.visit_bool(true)
            }
            b'f' => {
                self.parse_ident(b"false")?;
                visitor.visit_bool(false)
            }
            b'-' => self.deserialize_i64(visitor),
            b'0'..=b'9' => self.deserialize_u64(visitor),
            b'"' => self.deserialize_str(visitor),
            b'[' => {
                self.advance();
                visitor.visit_seq(SeqAccess::new(self))
            }
            b'{' => {
                self.advance();
                visitor.visit_map(MapAccess::new(self))
            }
            token => Err(Error::unexpected_token(
                "one of: ", // FIXME: include more info
                token, self.index,
            )),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.whitespace_or_eof()? {
            b't' => {
                self.parse_ident(b"true")?;
                visitor.visit_bool(true)
            }
            b'f' => {
                self.parse_ident(b"false")?;
                visitor.visit_bool(false)
            }
            token => Err(Error::unexpected_token(
                "one of: true, false",
                token,
                self.index,
            )),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.whitespace_or_eof()? {
            b'-' => {
                self.advance();
                visitor.visit_i64(self.parse_integer(true)?)
            }
            b'0'..=b'9' => visitor.visit_i64(self.parse_integer(false)?),
            _ => todo!(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_i64(visitor)
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
        match self.whitespace_or_eof()? {
            b'"' => {
                self.advance();

                let start = self.index;

                loop {
                    match self.next()? {
                        Some(byte) => {
                            if byte == b'"' {
                                break;
                            }
                        }
                        None => return Err(Error::Eof),
                    }
                }

                let end = self.index;

                let string =
                    std::str::from_utf8(&self.bytes[start..end]).map_err(|_| Error::InvalidUtf8)?;

                visitor.visit_str(string)
            }
            _ => todo!(),
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
        match self.parse_whitespace()? {
            Some(b'n') => {
                self.parse_ident(b"null")?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.whitespace_or_eof()? {
            b'n' => {
                self.parse_ident(b"null")?;
                visitor.visit_unit()
            }
            token => Err(Error::unexpected_token("null", token, self.index)),
        }
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
        match self.whitespace_or_eof()? {
            b'[' => {
                self.advance();
                visitor.visit_seq(SeqAccess::new(self))
            }
            token => Err(Error::unexpected_token("[", token, self.index)),
        }
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
        match self.whitespace_or_eof()? {
            b'{' => {
                self.advance();
                visitor.visit_map(MapAccess::new(self))
            }
            token => Err(Error::unexpected_token("{", token, self.index)),
        }
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
        self.deserialize_map(visitor)
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

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b']' => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b'}' => {
                self.de.advance();
                return Ok(None);
            }
            b'\'' => {
                todo!()
            }
            _ => {
                let start = self.de.index;

                loop {
                    match self.de.peek()? {
                        Some(byte) => {
                            if byte.is_ascii_whitespace() || matches!(byte, b'.' | b'=') {
                                break;
                            }

                            self.de.advance();
                        }
                        None => break,
                    }
                }

                let end = self.de.index;

                if start == end {
                    // return Err(Error::EmptyKey);
                }

                let key = std::str::from_utf8(&self.de.bytes[start..end])
                    .map_err(|_| Error::InvalidUtf8)?;

                seed.deserialize(de::value::StrDeserializer::new(key))
                    .map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.de.whitespace_or_eof()? {
            b'=' => {
                self.de.advance(); // Skip the equals sign
                seed.deserialize(&mut *self.de)
            }
            token => Err(Error::unexpected_token("=", token, self.de.index)),
        }
    }
}
