//! ZDoom patches.

use std::io::{Read, Seek, SeekFrom};
use std::ops::{Deref, DerefMut};

use bevy_color::{Color, Srgba};

use serde::{Deserialize, Deserializer};

use eyre::Report;

use derive_more::{Display, Error};

/// The amount of colors a DOOM palette has.
///
/// This is also the amount of unique `u8`s there are.
pub const PALETTE_COLORS: usize = 1 << 8;

/// The "end-of-column" byte for posts.
const END_OF_COLUMN: u8 = 0xFF;

/// A patch.
pub struct Patch {
    /// The width of the patch.
    pub width: usize,
    /// The height of the patch.
    pub height: usize,
    /// The left offset of the patch.
    pub left: i32,
    /// The top offset of the patch.
    pub top: i32,
    /// The image data, represented as a flattened, 2D array of palette
    /// indices.
    pub image: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct PatchHeader {
    width: u16,
    height: u16,
    left_offset: i16,
    top_offset: i16,
}

impl Patch {
    /// Reads a patch.
    pub fn read<R>(mut reader: R) -> Result<Patch, Report>
    where
        R: Read + Seek,
    {
        // read header
        let header: PatchHeader = bincode::deserialize_from(&mut reader)?;

        let width = header.width as usize;
        let height = header.height as usize;

        // now read image data
        let image = (0..width * height).map(|_| u8::MAX).collect::<Vec<u8>>();

        let mut image = Patch {
            width,
            height,
            left: header.left_offset as i32,
            top: header.top_offset as i32,
            image,
        };

        // iterate over our posts
        for x in 0..width {
            // +8 to skip header
            reader.seek(SeekFrom::Start((x * 4 + 8) as u64))?;
            let offset: u32 = bincode::deserialize_from(&mut reader)?;

            // start from offset
            reader.seek(SeekFrom::Start(offset as u64))?;

            // the current y offset of the current post
            let mut top_delta = 0;

            while next_post_offset(&mut reader, &mut top_delta)? {
                // read post length
                let length: u8 = bincode::deserialize_from(&mut reader)?;
                // skip buffer byte
                reader.seek(SeekFrom::Current(1))?;

                // read post data
                for y in 0..(length as usize) {
                    // get true y of post
                    let y = top_delta + y;

                    if y >= height as usize {
                        continue;
                    }

                    let palette_ix: u8 = bincode::deserialize_from(&mut reader)?;

                    let color_offset = (x + y * header.width as usize) * 4;
                    image.image[color_offset] = palette_ix;
                }

                // skip buffer byte
                reader.seek(SeekFrom::Current(1))?;
            }
        }

        Ok(image)
    }
}

/// Returns `false` if there are no more posts.
fn next_post_offset<R>(reader: R, top_delta: &mut usize) -> Result<bool, Report>
where
    R: Read,
{
    // read byte offset
    let byte: u8 = bincode::deserialize_from(reader)?;

    if byte != END_OF_COLUMN {
        // more data to read, check if we are doing a tall patch
        let offset = byte as usize;

        if offset <= *top_delta {
            // add offset to topdelta
            *top_delta += offset;
        } else {
            *top_delta = offset;
        }

        Ok(true)
    } else {
        // thjs is the last post
        Ok(false)
    }
}

/// A single color palette.
///
/// A palette has a serialize/deserialize implementation that serializes it
/// into an array of bytes. The palette can be indexed as expected.
#[derive(Clone, Debug)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    /// Creates a palette from a densely-packed list of colors.
    pub fn from_bytes(buf: &[u8]) -> Result<Palette, InvalidPaletteLength> {
        if buf.len() != PALETTE_COLORS * 3 {
            return Err(InvalidPaletteLength(buf.len()));
        }

        let mut colors = Vec::with_capacity(PALETTE_COLORS);

        for start_ix in (0..PALETTE_COLORS).map(|ix| ix * 3) {
            let mut color = [0u8; 3];
            color.copy_from_slice(&buf[start_ix..start_ix + 3]);

            let [r, g, b] = color;

            colors.push(
                Srgba {
                    red: r as f32 / 255.,
                    green: g as f32 / 255.,
                    blue: b as f32 / 255.,
                    alpha: 1.,
                }
                .into(),
            );
        }

        Ok(Palette { colors })
    }
}

impl Deref for Palette {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Palette {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl<'de> Deserialize<'de> for Palette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = <&[u8]>::deserialize(deserializer)?;
        Palette::from_bytes(buf).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Default for Palette {
    fn default() -> Palette {
        const PLAYPAL: &[u8] = include_bytes!("PLAYPAL.pal");

        // only return first node
        Palette::from_bytes(&PLAYPAL[..PALETTE_COLORS * 3]).expect("valid default PLAYPAL")
    }
}

/// An error for [`Palette::from_bytes`].
#[derive(Debug, Display, Error)]
#[display("invalid palette len: {_0}")]
pub struct InvalidPaletteLength(#[error(not(source))] pub usize);
