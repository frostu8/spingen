//! Shared, "standardized" value parsing.

use serde::{
    de::{self, Deserialize, DeserializeSeed, Deserializer, Visitor},
    forward_to_deserialize_any,
};

use std::num::{ParseFloatError, ParseIntError};

use derive_more::{Display, From};

/// Wraps a string value, exposing some common, obvious parsing details.
///
/// Internally, Ring Racers uses the same syntax for SOC value parsing.
pub struct ValueDeserializer<'de>(&'de str);

impl<'de> ValueDeserializer<'de> {
    /// Creates a new `ValueDeserializer`.
    pub fn new(input: &'de str) -> ValueDeserializer<'de> {
        // ignore whitespace
        ValueDeserializer(input.trim())
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.0)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.0.to_owned())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<u8>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_u8(data))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<u16>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_u16(data))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<u32>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_u32(data))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<u64>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_u64(data))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<u128>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_u128(data))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<i8>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_i8(data))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<i16>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_i16(data))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<i32>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_i32(data))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<i64>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_i64(data))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<i128>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_i128(data))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<f32>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_f32(data))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0
            .parse::<f64>()
            .map_err(From::from)
            .and_then(|data| visitor.visit_f64(data))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.0.chars().count() == 1 {
            let ch = self.0.chars().next().unwrap();
            visitor.visit_char(ch)
        } else {
            Err(Error {
                inner: ErrorKind::InvalidLength(self.0.len()),
            })
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.0.eq_ignore_ascii_case("TRUE")
            || self.0.eq_ignore_ascii_case("YES")
            || self.0.eq_ignore_ascii_case("1")
        {
            visitor.visit_bool(true)
        } else if self.0.eq_ignore_ascii_case("FALSE")
            || self.0.eq_ignore_ascii_case("NO")
            || self.0.eq_ignore_ascii_case("0")
        {
            visitor.visit_bool(false)
        } else {
            Err(Error {
                inner: ErrorKind::InvalidBoolean(self.0.to_string()),
            })
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    forward_to_deserialize_any! {
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> de::SeqAccess<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.0.len() > 0 {
            // get next value in comma seperated list
            if let Some((value, next)) = self.0.split_once(',') {
                self.0 = next;
                seed.deserialize(ValueDeserializer(value)).map(|s| Some(s))
            } else {
                let value = self.0;
                self.0 = "";
                seed.deserialize(ValueDeserializer(value)).map(|s| Some(s))
            }
        } else {
            Ok(None)
        }
    }
}

/// A deserializer error.
#[derive(Debug, Display)]
#[display("{inner}")]
pub struct Error {
    inner: ErrorKind,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner {
            ErrorKind::ParseInt(parse) => Some(parse),
            ErrorKind::ParseFloat(parse) => Some(parse),
            _ => None,
        }
    }
}

impl<T> From<T> for Error
where
    ErrorKind: From<T>,
{
    fn from(value: T) -> Self {
        Error {
            inner: value.into(),
        }
    }
}

#[derive(Debug, Display, From)]
pub enum ErrorKind {
    #[display("invalid len: {_0}")]
    #[from(ignore)]
    InvalidLength(usize),
    #[display("expeced a bool, got \"{_0}\"")]
    #[from(ignore)]
    InvalidBoolean(String),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    #[display("{_0}")]
    #[from(ignore)]
    Message(String),
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            inner: ErrorKind::Message(msg.to_string()),
        }
    }
}

/// Deserializes a string into a useful type.
pub fn deserialize<'a, T>(input: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    T::deserialize(ValueDeserializer::new(input))
}

/// Deserializes a string into a useful type.
pub fn deserialize_seed<'a, T, U>(input: &'a str, seed: T) -> Result<U, Error>
where
    T: DeserializeSeed<'a, Value = U>,
{
    seed.deserialize(ValueDeserializer::new(input))
}
