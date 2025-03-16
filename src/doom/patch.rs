//! ZDoom patches.

use std::io::{Read, Seek, SeekFrom};
use std::ops::{Deref, DerefMut};

use bevy_color::color_difference::EuclideanDistance;
use bevy_color::{Color, Srgba};

use serde::{Deserialize, Deserializer};

use derive_more::{Display, Error};

/// The amount of colors a DOOM palette has.
///
/// This is also the amount of unique `u8`s there are.
pub const PALETTE_COLORS: usize = 1 << 8;

/// The "end-of-column" byte for posts.
const END_OF_COLUMN: u8 = 0xFF;

/// A patch.
#[derive(Clone, Debug)]
pub struct Patch {
    /// The left offset of the patch.
    pub left: i32,
    /// The top offset of the patch.
    pub top: i32,
    /// The width of the patch.
    pub width: u16,
    /// The height of the patch.
    pub height: u16,
    /// The internal data of the patch.
    ///
    /// Stored as a 2D array.
    pub data: Vec<Option<u8>>,
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
    pub fn read<R>(mut reader: R) -> Result<Patch, Error>
    where
        R: Read + Seek,
    {
        // read header
        let header: PatchHeader = bincode::deserialize_from(&mut reader)?;

        let width = header.width as usize;
        let height = header.height as usize;

        // now read image data
        let mut data = (0..width * height)
            .map(|_| None)
            .collect::<Vec<Option<u8>>>();

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

                    // fill palette index data
                    let color_offset = x + y * width;
                    data[color_offset] = Some(palette_ix);
                }

                // skip buffer byte
                reader.seek(SeekFrom::Current(1))?;
            }
        }

        // create image
        Ok(Patch {
            left: header.left_offset as i32,
            top: header.top_offset as i32,
            width: header.width,
            height: header.height,
            data,
        })
    }
}

/// Returns `false` if there are no more posts.
fn next_post_offset<R>(reader: R, top_delta: &mut usize) -> Result<bool, Error>
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
    colors: [Color; PALETTE_COLORS],
}

impl Palette {
    /// Creates a palette from a densely-packed list of colors.
    pub fn from_bytes(buf: &[u8]) -> Result<Palette, InvalidPaletteLength> {
        if buf.len() != PALETTE_COLORS * 3 {
            return Err(InvalidPaletteLength(buf.len()));
        }

        let mut colors = [Color::default(); PALETTE_COLORS];

        for i in 0..PALETTE_COLORS {
            let start_ix = i * 3;
            let mut color = [0u8; 3];
            color.copy_from_slice(&buf[start_ix..start_ix + 3]);

            let [r, g, b] = color;
            let color: Color = Srgba {
                red: r as f32 / 255.,
                green: g as f32 / 255.,
                blue: b as f32 / 255.,
                alpha: 1.,
            }
            .into();

            colors[i] = color;
        }

        Ok(Palette { colors })
    }

    /// Finds the nearest color to another using euclidian sRGBA distances.
    pub fn nearest_color(&self, color: Color) -> usize {
        let mut min_color_ix = 0;
        let mut min_distance = self.colors[min_color_ix].distance_squared(&color);

        for (i, pal_color) in self.colors.iter().enumerate().skip(1) {
            let distance = pal_color.distance_squared(&color);

            if distance < min_distance {
                min_color_ix = i;
                min_distance = distance;
            }
        }

        min_color_ix
    }

    /// Copies the color values of an index into the buffer as [`Srgba`].
    ///
    /// # Panics
    /// Panics if `buf` is not len 3 or higher, or if `ix` is out of bounds.
    pub fn copy_color(&self, ix: usize, buf: &mut [u8]) {
        assert!(buf.len() >= 3);

        let color = self[ix].to_srgba();

        buf[0] = (color.red * 255.) as u8;
        buf[1] = (color.green * 255.) as u8;
        buf[2] = (color.blue * 255.) as u8;
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

/// An error for patch reading.
#[derive(Debug, Display, Error)]
pub enum Error {
    Deser(bincode::Error),
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self {
        Error::Deser(value)
    }
}

/// An error for [`Palette::from_bytes`].
#[derive(Debug, Display, Error)]
#[display("invalid palette len: {_0}")]
pub struct InvalidPaletteLength(#[error(not(source))] pub usize);
