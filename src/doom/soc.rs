//! SOC deserialization.
//!
//! There isn't exactly a rigid SOC definition, so this is just a best-effort
//! parser.

use std::cmp::min;

use serde::{
    de::{self, value::StringDeserializer, Deserialize, Error as _},
    forward_to_deserialize_any,
};

use crate::doom::value::{self, ValueDeserializer};

use derive_more::Display;

/// An SOC parser.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    input: &'a str,
    in_freeslot: bool,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser`.
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            input,
            in_freeslot: false,
        }
    }

    /// Deserializes everything up to the end of the block as a type.
    pub fn deserialize<T>(&mut self) -> Result<T, Error>
    where
        T: Deserialize<'a>,
    {
        let deserializer = Deserializer::new(self);
        T::deserialize(deserializer)
    }

    fn line_ended(&mut self) -> bool {
        let (cont, ix) = scan_whitespace(self.input);
        self.input = &self.input[min(ix, self.input.len())..];
        !cont
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.input.len() > 0 {
            // find next meaningful line
            if self.line_ended() {
                continue;
            }

            // we should now be at the next ident
            let ix = scan_while(self.input, is_ident);
            let ident = &self.input[..ix];
            self.input = &self.input[ix..];

            // skip whitespace
            if self.line_ended() {
                if ident.eq_ignore_ascii_case("FREESLOT") {
                    // this begins a list of freeslots
                    self.in_freeslot = true;
                } else if self.in_freeslot {
                    // this is a freeslot definition
                    return Some(Event::Freeslot(ident));
                } else {
                    // this is a header with no value
                    return Some(Event::Header {
                        name: ident,
                        value: None,
                    });
                }
            } else if matches!(self.input.as_bytes(), [b'=', ..]) {
                self.input = &self.input[1..];

                // this is an assignment operator
                if self.line_ended() {
                    // REGRESSION: return blank value if there is nothing left
                    // on the line
                    return Some(Event::KeyValue {
                        name: ident,
                        value: "",
                    });
                } else {
                    // scan rest of line
                    let end_ix = scan_while(self.input, |byte| *byte != b'#' && *byte != b'\n');
                    let value = &self.input[..end_ix].trim();
                    self.input = &self.input[min(end_ix + 1, self.input.len())..]; // + 1 to skip end char
                    return Some(Event::KeyValue { name: ident, value });
                }
            } else {
                // this is a header with a value
                self.in_freeslot = false;
                // scan rest of line
                let end_ix = scan_while(self.input, |byte| *byte != b'#' && *byte != b'\n');
                let value = &self.input[..end_ix].trim();
                self.input = &self.input[min(end_ix + 1, self.input.len())..]; // + 1 to skip end char
                return Some(Event::Header {
                    name: ident,
                    value: Some(value),
                });
            }
        }

        None
    }
}

/// An SOC event.
#[derive(Clone, Debug, PartialEq)]
pub enum Event<'a> {
    /// A freeslot definition.
    Freeslot(&'a str),
    /// A header, with an optional header value.
    Header {
        name: &'a str,
        value: Option<&'a str>,
    },
    /// A key-value pairing.
    KeyValue { name: &'a str, value: &'a str },
}

/// A block deserializer.
///
/// This is so that serde macros can be used to read SOC files. I am lazy. This
/// is by no means meant to be a full implementation like the UDMF stuff.
pub struct Deserializer<'a, 'de> {
    parser: &'a mut Parser<'de>,
    value: Option<&'de str>,
}

impl<'a, 'de> Deserializer<'a, 'de> {
    /// Creates a new `Deserializer`.
    pub fn new(parser: &'a mut Parser<'de>) -> Deserializer<'a, 'de> {
        Deserializer {
            parser,
            value: None,
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'a, 'de> de::MapAccess<'de> for Deserializer<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        // rudimentary peeking, this isn't really crazy
        let saved_parser = self.parser.clone();

        match self.parser.next() {
            Some(Event::KeyValue { name, value }) => {
                // save value
                self.value = Some(value);

                // convert name to lowercase
                let name = name.to_ascii_lowercase();
                seed.deserialize(StringDeserializer::new(name))
                    .map(|s| Some(s))
            }
            _ => {
                // reset parser
                *self.parser = saved_parser;
                Ok(None)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let Some(value) = self.value.take() else {
            return Err(Error::custom("next_value called before next_key"));
        };

        seed.deserialize(ValueDeserializer::new(value))
            .map_err(From::from)
    }
}

/// A deserializer error.
#[derive(Debug, Display)]
#[display("{kind}")]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    #[display("invalid value")]
    Value(value::Error),
    Message(String),
}

impl From<value::Error> for Error {
    fn from(value: value::Error) -> Self {
        Error {
            kind: ErrorKind::Value(value),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Value(err) => Some(err),
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

/// Scans the input for whitespace.
///
/// Returns the index of the next non-whitespace character on the line. If the
/// line ends, or a comment is reached, returns the start index of the next
/// line, with the `continue` flag `false`.
fn scan_whitespace(input: &str) -> (bool, usize) {
    let mut ix = 0;

    while ix < input.len() {
        // skip whitespace first
        ix += scan_while(&input[ix..], |byte| {
            *byte != b'\n' && byte.is_ascii_whitespace()
        });

        match &input.as_bytes()[ix..] {
            [b'#', ..] => {
                // this is a comment, scan until newline
                ix += scan_while(&input[ix..], |byte| *byte != b'\n');
                return (false, ix + 1);
            }
            // the line has ended
            [b'\n', ..] => return (false, ix + 1),
            _ => break,
        }
    }

    (true, ix)
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

fn is_ident(byte: &u8) -> bool {
    byte.is_ascii_alphabetic() || *byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soc() {
        const INPUT: &str = r#"
FREESLOT
SKINCOLOR_VENUS # this is a comment
# this is a comment on a newline

SKINCOLOR SKINCOLOR_VENUS
NAME = Venus
RAMP = 171,171,172,172,173,173,174,174,174,175,175,175,139,139,29,29
INVCOLOR = SKINCOLOR_SLATE
INVSHADE = 14
CHATCOLOR = V_PURPLEMAP
ACCESSIBLE = TRUE"#;

        let parser = Parser::new(INPUT);
        let events = parser.collect::<Vec<_>>();

        assert_eq!(
            events,
            &[
                Event::Freeslot("SKINCOLOR_VENUS"),
                Event::Header {
                    name: "SKINCOLOR",
                    value: Some("SKINCOLOR_VENUS"),
                },
                Event::KeyValue {
                    name: "NAME",
                    value: "Venus"
                },
                Event::KeyValue {
                    name: "RAMP",
                    value: "171,171,172,172,173,173,174,174,174,175,175,175,139,139,29,29"
                },
                Event::KeyValue {
                    name: "INVCOLOR",
                    value: "SKINCOLOR_SLATE"
                },
                Event::KeyValue {
                    name: "INVSHADE",
                    value: "14"
                },
                Event::KeyValue {
                    name: "CHATCOLOR",
                    value: "V_PURPLEMAP"
                },
                Event::KeyValue {
                    name: "ACCESSIBLE",
                    value: "TRUE"
                },
            ],
        )
    }
}
