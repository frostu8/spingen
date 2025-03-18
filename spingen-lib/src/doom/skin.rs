//! ZDoom skins, and skin definitions.

use derive_more::{Display, Error};

use super::value::{deserialize, Error as ValueError};

/// A skin definition.
#[derive(Clone, Debug)]
pub struct SkinDefine {
    /// The name that identifies this skin.
    pub name: String,
    /// The real, display name of the skin.
    ///
    /// Note: In Ring Racers, underscores in this name will be replaced with
    /// spaces in UI, so `spingen` will replace any underscores with spaces.
    pub realname: String,
    /// The start color for spray replacement.
    pub startcolor: u8,
    /// The preferred color of the racer.
    ///
    /// In `spingen`, it will automatically select this color.
    pub prefcolor: String,
    /// The kart speed.
    ///
    /// Default is `5`.
    pub kartspeed: i32,
    /// The kart weight.
    ///
    /// Default is `5`.
    pub kartweight: i32,
}

impl SkinDefine {
    /// Reads a skin define from a lump.
    pub fn read(input: &str) -> Result<SkinDefine, Error> {
        let mut name = None::<String>;
        let mut realname = None::<String>;
        let mut startcolor = default_startcolor();
        let mut prefcolor = None::<String>;

        // we are also looking for class information, just if a user might want
        // to see their class as well.
        let mut kartspeed = 5;
        let mut kartweight = 5;

        for (line_no, line) in input.lines().enumerate().map(|(no, inner)| (no + 1, inner)) {
            let Some(ix) = line.find('=') else {
                return Err(Error {
                    kind: ErrorKind::MissingValue(Position {
                        line: line_no,
                        col: line.chars().count(),
                    }),
                });
            };
            let (key, rest) = line.split_at(ix);

            let rest = &rest[1..]; // skip '='
            let key = key.trim();

            let parse_err = |err| Error {
                kind: ErrorKind::InvalidValue(
                    Position {
                        // +1 to skip over '='
                        col: line[..ix].chars().count() + 1,
                        line: line_no,
                    },
                    err,
                ),
            };

            if key.eq_ignore_ascii_case("name") {
                name = Some(deserialize(rest).map_err(parse_err)?);
            } else if key.eq_ignore_ascii_case("realname") {
                realname = Some(deserialize(rest).map_err(parse_err)?);
            } else if key.eq_ignore_ascii_case("prefcolor") {
                prefcolor = Some(deserialize(rest).map_err(parse_err)?);
            } else if key.eq_ignore_ascii_case("startcolor") {
                startcolor = deserialize(rest).map_err(parse_err)?;
            } else if key.eq_ignore_ascii_case("kartspeed") {
                kartspeed = deserialize(rest).map_err(parse_err)?;
            } else if key.eq_ignore_ascii_case("kartweight") {
                kartweight = deserialize(rest).map_err(parse_err)?;
            }
        }

        Ok(SkinDefine {
            name: name.ok_or_else(|| Error::missing_field("name"))?,
            realname: realname.ok_or_else(|| Error::missing_field("realname"))?,
            prefcolor: prefcolor.ok_or_else(|| Error::missing_field("prefcolor"))?,
            startcolor,
            kartspeed,
            kartweight,
        })
    }
}

/// An error for loading a skin define.
#[derive(Debug, Display, Error)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    fn missing_field(field: &'static str) -> Error {
        Error {
            kind: ErrorKind::MissingField(field),
        }
    }
}

/// A position in a file.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[display("{line}:{col}")]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    /// The define is missing a required field.
    #[display("missing field \"{_0}\"")]
    MissingField(&'static str),
    /// A value pailed to parse
    #[display("@ {_0} {_1}")]
    InvalidValue(Position, ValueError),
    /// The define does not have a value.
    #[display("@ {_0} expected '='")]
    MissingValue(Position),
}

fn default_startcolor() -> u8 {
    96
}
