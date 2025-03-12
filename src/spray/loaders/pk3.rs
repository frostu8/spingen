//! PK3 loaders for spraycans.
//!
//! Wads cannot carry spray information?

use std::io::{Cursor, Read};
use std::path::Path;

use bytes::Bytes;

use ahash::HashMap;

use serde::Deserialize;

use zip::ZipArchive;

use eyre::{Report, WrapErr};

use crate::doom::{
    soc::{Event, Parser},
    spray::Spray as DoomSpray,
};
use crate::spray::Spray;

/// A PK3 spray loader.
#[derive(Clone, Debug)]
pub struct Pk3SprayLoader {
    zip: ZipArchive<Cursor<Bytes>>,
    file_index: usize,
    sprays: HashMap<String, DoomSpray>,
}

impl Pk3SprayLoader {
    /// Creates a new spray loader.
    pub fn new(bytes: impl Into<Bytes>) -> Result<Pk3SprayLoader, Report> {
        let bytes = bytes.into();
        ZipArchive::new(Cursor::new(bytes))
            .map(|zip| Pk3SprayLoader {
                zip,
                file_index: 0,
                sprays: HashMap::default(),
            })
            .map_err(From::from)
    }

    fn read_soc(&mut self, ix: usize) -> Result<(), Report> {
        let mut entry = self.zip.by_index(ix)?;
        let mut text = String::new();
        entry
            .read_to_string(&mut text)
            .wrap_err_with(|| format!("failed reading SOC \"{}\"", entry.name()))?;

        let wrap_err = || format!("failed reading SOC \"{}\"", entry.name());

        // open soc with parser
        let mut parser = Parser::new(&text);

        while let Some(ev) = parser.next() {
            match ev {
                Event::Freeslot(name) if is_skincolor_name(name) => {
                    leptos::logging::log!("{:?}", name);
                    self.sprays
                        .entry(name.to_owned())
                        .or_insert_with_key(|key| DoomSpray {
                            id: key.clone(),
                            ..Default::default()
                        });
                }
                Event::Header {
                    name,
                    value: Some(value),
                } if name.eq_ignore_ascii_case("SKINCOLOR") && is_skincolor_name(value) => {
                    leptos::logging::log!("{:?}", self.sprays);
                    let Some(spray) = self.sprays.get_mut(value) else {
                        return Err(Report::msg(format!("undefined skincolor: \"{}\"", value)))
                            .wrap_err_with(wrap_err);
                    };

                    let deser_spray = parser
                        .deserialize::<OptionalSpray>()
                        .wrap_err_with(wrap_err)?;

                    if let Some(name) = deser_spray.name {
                        spray.name = name;
                    }
                    if let Some(ramp) = deser_spray.ramp {
                        spray.ramp = ramp;
                    }
                }
                // skip unknonwn directives
                _ => (),
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct OptionalSpray {
    pub name: Option<String>,
    pub ramp: Option<[u8; 16]>,
}

fn is_skincolor_name(name: &str) -> bool {
    const PREFIX: &str = "SKINCOLOR_";

    if name.len() >= PREFIX.len() {
        name[..PREFIX.len()].eq_ignore_ascii_case(PREFIX)
    } else {
        false
    }
}

impl Iterator for Pk3SprayLoader {
    type Item = Result<Spray, Report>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.file_index < self.zip.len() {
            let file_index = self.file_index;
            self.file_index += 1;

            // find either an soc or a lua file
            let entry = match self.zip.by_index_raw(file_index) {
                Ok(entry) => entry,
                Err(err) => return Some(Err(err.into())),
            };

            // check tld
            let path = Path::new(entry.name());

            match path
                .components()
                .next()
                .map(|s| s.as_os_str().to_str().expect("path from str"))
            {
                Some(tld) if tld.eq_ignore_ascii_case("lua") => {
                    // TODO: impl lua reading, ignore for now
                    ()
                }
                Some(tld) if tld.eq_ignore_ascii_case("soc") => {
                    drop(entry);
                    // load soc
                    if let Err(err) = self.read_soc(file_index) {
                        return Some(Err(err.into()));
                    };
                }
                // other file, ignore
                Some(_) => (),
                None => (),
            }
        }

        if let Some(key) = self.sprays.keys().next().cloned() {
            let spray = self.sprays.remove(&key).expect("present value");
            Some(Ok(spray.into()))
        } else {
            None
        }
    }
}
