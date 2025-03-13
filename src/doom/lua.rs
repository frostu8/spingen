//! Limited BLua reading things.

use serde::de::value::{BorrowedStrDeserializer, UsizeDeserializer};
use serde::de::{self, Deserializer, IgnoredAny, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

use std::borrow::Cow;

use derive_more::Display;

/// A BLua literal deserializer.
#[derive(Clone, Debug)]
pub struct LiteralDeserializer<'de> {
    input: &'de str,
}

impl<'de> LiteralDeserializer<'de> {
    /// Creates a new `LiteralDeserializer`.
    pub fn new(input: &'de str) -> LiteralDeserializer<'de> {
        LiteralDeserializer { input }
    }

    fn inner_any<V>(
        mut self,
        visitor: V,
        hint: SeqHint,
    ) -> Result<V::Value, <Self as Deserializer<'de>>::Error>
    where
        V: Visitor<'de>,
    {
        self.skip_whitespace();

        if self.input.is_empty() {
            return Err(Error {
                kind: ErrorKind::Eof,
            });
        }

        // Thankfully, in BLua the syntax for numbers are limited because of
        // fixed-point math. The only types we really have to worry about are
        // integers and strings.
        if matches!(self.input.as_bytes(), [b'"', ..] | [b'\'', ..]) {
            let (data, ix) = scan_string(self.input);
            self.input = &self.input[ix..];

            match data {
                Cow::Borrowed(data) => visitor.visit_borrowed_str(data),
                Cow::Owned(data) => visitor.visit_string(data),
            }
        } else if self.input.as_bytes()[0].is_ascii_digit() {
            // scan all digits
            let ix = scan_while(self.input, u8::is_ascii_digit);
            let number = self.input[..ix].parse::<i32>().map_err(|err| Error {
                kind: ErrorKind::ParseInt(err),
            })?;
            self.input = &self.input[ix..];

            visitor.visit_i32(number)
        } else if matches!(self.input.as_bytes(), [b'{', ..]) {
            self.input = &self.input[1..];
            // this is a table
            match hint {
                SeqHint::Map => visitor.visit_map(LiteralMapAccess::from(self)),
                SeqHint::Seq => visitor.visit_seq(LiteralMapAccess::from(self)),
            }
        } else {
            // this is a constant
            let ix = scan_while(self.input, |byte| !byte.is_ascii_whitespace());
            let name = &self.input[..ix];
            self.input = &self.input[ix..];

            match name {
                b if b.eq_ignore_ascii_case("true") => visitor.visit_bool(true),
                b if b.eq_ignore_ascii_case("false") => visitor.visit_bool(false),
                name => visitor.visit_borrowed_str(name),
            }
        }
    }

    fn skip_whitespace(&mut self) {
        let ix = scan_whitespace(&self.input);
        self.input = &self.input[ix..];
    }
}

impl<'de> Deserializer<'de> for LiteralDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner_any(visitor, SeqHint::Map)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner_any(visitor, SeqHint::Seq)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct
        tuple_struct map struct enum identifier ignored_any
    }
}

enum SeqHint {
    Seq,
    Map,
}

#[derive(Debug)]
struct LiteralMapAccess<'de> {
    input: &'de str,
    index: usize,
}

impl<'de> LiteralMapAccess<'de> {
    fn skip_whitespace(&mut self) {
        let ix = scan_whitespace(&self.input);
        self.input = &self.input[ix..];
    }
}

impl<'de> From<LiteralDeserializer<'de>> for LiteralMapAccess<'de> {
    fn from(value: LiteralDeserializer<'de>) -> Self {
        LiteralMapAccess {
            input: value.input,
            index: 0,
        }
    }
}

impl<'de> de::MapAccess<'de> for LiteralMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.skip_whitespace();

        // find end of entry
        let mut depth = 0;
        let ix = scan_while(self.input, |byte| {
            if depth == 0 && (*byte == b'}' || *byte == b',') {
                return false;
            }

            if *byte == b'}' {
                if depth > 0 {
                    depth -= 1;
                }
            }

            if *byte == b'{' {
                depth += 1;
            }

            *byte != b'='
        });

        // check if this is the end of the map
        if ix == 0 && self.input.as_bytes()[ix] == b'}' {
            return Ok(None);
        }

        match &self.input.as_bytes()[ix..] {
            [b'=', ..] => {
                // take key
                let key = self.input[..ix].trim();
                self.input = &self.input[ix + 1..]; // +1 to skip =
                self.skip_whitespace();
                seed.deserialize(BorrowedStrDeserializer::new(key))
                    .map(Some)
            }
            _ => {
                // return next index
                let index = self.index;
                self.index += 1;
                seed.deserialize(UsizeDeserializer::new(index)).map(Some)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        // find end of entry
        let mut depth = 0;
        let ix = scan_while(self.input, |byte| {
            if depth == 0 && (*byte == b'}' || *byte == b',') {
                return false;
            }

            if *byte == b'}' {
                if depth > 0 {
                    depth -= 1;
                }
            }

            if *byte == b'{' {
                depth += 1;
            }

            true
        });

        // try to read new data
        let data = &self.input[..ix];
        self.input = &self.input[ix..];

        if self.input.len() > 0 && self.input.as_bytes()[0] == b',' {
            // skip comma
            self.input = &self.input[1..];
        }

        seed.deserialize(LiteralDeserializer::new(data))
    }
}

impl<'de> de::SeqAccess<'de> for LiteralMapAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.next_key::<IgnoredAny>() {
            Ok(Some(_)) => self.next_value_seed(seed).map(Some),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Display)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    Message(String),
    #[display("invalid literal encountered")]
    InvalidLiteral,
    ParseInt(std::num::ParseIntError),
    #[display("end-of-file reached")]
    Eof,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::ParseInt(err) => Some(err),
            _ => None,
        }
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error {
            kind: ErrorKind::Message(msg.to_string()),
        }
    }
}

fn scan_string<'a>(mut input: &'a str) -> (Cow<'a, str>, usize) {
    let open = input.as_bytes()[0];
    input = &input[1..];

    let mut ix = 0;

    let mut mark = 0;
    let mut buf = String::new();

    // this is an opening quote, search for the closing quotes
    while ix < input.len() {
        ix += scan_while(&input[ix..], |b| *b != open && *b != b'\\');

        if ix + 1 < input.len() && input.as_bytes()[ix] == b'\\' {
            // escape char copy everything from mark
            buf.push_str(&input[mark..ix]);
            match input.as_bytes()[ix + 1] {
                // TODO: maybe expand this list? this isn't really
                // relevant for spray cans, nobody is gonna write something
                // like:
                // name = "Troll\tCan"
                b'n' => buf.push('\n'),
                ch => buf.push(ch as char),
            }

            ix += 2;
            mark = ix;
        } else {
            break;
        }
    }

    // copy everything from mark if buf is in use
    if mark > 0 {
        buf.push_str(&input[mark..ix]);
        (Cow::Owned(buf), ix + 1)
    } else {
        (Cow::Borrowed(&input[..ix]), ix + 1) // +1 to skip end quote
    }
}

/// Scans the input for whitespace and comments.
fn scan_whitespace(input: &str) -> usize {
    let mut ix = 0;

    while ix < input.len() {
        // skip whitespace first
        ix += scan_while(&input[ix..], u8::is_ascii_whitespace);

        match &input.as_bytes()[ix..] {
            [b'-', b'-', b'[', b'[', ..] => {
                // this is the beginning of a multiline comment
                while ix < input.len() {
                    ix += scan_while(&input[ix..], |byte| *byte != b']');

                    if matches!(&input.as_bytes()[ix..], [b']', b']', ..]) {
                        break;
                    }
                }
            }
            [b'-', b'-', ..] => {
                // this is a single line comment
                ix += scan_while(&input[ix..], |byte| *byte != b'\n');
            }
            // something interesting encountered! break
            _ => break,
        }
    }

    ix
}

/// Scans the input while a particular condition holds for each character.
fn scan_while<'a, F>(input: &'a str, mut cond: F) -> usize
where
    F: FnMut(&'a u8) -> bool,
{
    let input = input.as_bytes();
    let mut i = 0;

    while i < input.len() {
        if !cond(&input[i]) {
            break;
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;

    #[test]
    fn test_deser() {
        const INPUT: &str = r#"
{
name = "Asimov",
ramp = {0,1,3,5,6,8,9,134,135,148,149,137,26,27,28,29},
invcolor = SKINCOLOR_PERIWINKLE,
invshade = 7,
chatcolor = V_BLUEMAP,
accessible = true
}
        "#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Spray {
            name: String,
            ramp: [u8; 16],
            invcolor: String,
            invshade: u8,
            chatcolor: String,
            accessible: bool,
        }

        let deser = LiteralDeserializer::new(INPUT);
        let value = Spray::deserialize(deser).unwrap();

        assert_eq!(
            value,
            Spray {
                name: "Asimov".into(),
                ramp: [0, 1, 3, 5, 6, 8, 9, 134, 135, 148, 149, 137, 26, 27, 28, 29],
                invcolor: "SKINCOLOR_PERIWINKLE".into(),
                invshade: 7,
                chatcolor: "V_BLUEMAP".into(),
                accessible: true,
            }
        );
    }

    #[test]
    fn test_values() {
        let deser = LiteralDeserializer::new("101");
        let value = u8::deserialize(deser).unwrap();
        assert_eq!(value, 101);

        let deser = LiteralDeserializer::new(
            r#"
            "\"Super\" \nLuigi!"   
        "#,
        );
        let value = String::deserialize(deser).unwrap();
        assert_eq!(value, "\"Super\" \nLuigi!");
    }
}
