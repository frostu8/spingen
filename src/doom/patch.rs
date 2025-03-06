//! ZDoom patches.

use std::io::{Read, Seek, SeekFrom, Write};
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
#[derive(Clone, Debug)]
pub struct Patch {
    /// The left offset of the patch.
    pub left: i32,
    /// The top offset of the patch.
    pub top: i32,
    /// The image data.
    pub image: Image,
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
    pub fn read<R>(reader: R) -> Result<Patch, Report>
    where
        R: Read + Seek,
    {
        Patch::read_with(reader, &Palette::default())
    }

    /// Reads a patch with a given palette.
    pub fn read_with<R>(mut reader: R, palette: &Palette) -> Result<Patch, Report>
    where
        R: Read + Seek,
    {
        // read header
        let header: PatchHeader = bincode::deserialize_from(&mut reader)?;

        let width = header.width as usize;
        let height = header.height as usize;

        // now read image data
        let mut data = (0..width * height)
            .flat_map(|_| {
                // make all transparent pixels cyan
                // yes, I know cyan doesn't make it transparent, but this skips
                // an indirecton for when flats are rendered later in geometry
                let mut color = [0u8; 4];
                palette.copy_color(255, &mut color[..3]);
                color.into_iter()
            })
            .collect::<Vec<u8>>();

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
                    let color_data = &mut data[color_offset..color_offset + 4];

                    // mark pixel as occupied
                    color_data[3] = u8::MAX;

                    // fill color data
                    palette.copy_color(palette_ix as usize, &mut color_data[..3]);
                }

                // skip buffer byte
                reader.seek(SeekFrom::Current(1))?;
            }
        }

        // create image
        let image = Image {
            width: width as u32,
            height: height as u32,
            data,
        };

        Ok(Patch {
            left: header.left_offset as i32,
            top: header.top_offset as i32,
            image,
        })
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

/// The image portion of a patch.
///
/// The data here is stored as a flattened array of `SrgbaUnorm`. The size of
/// data, if the image is well-formed, will be `width * height * 4`.
///
/// This is *most of the time* the data you care about.
#[derive(Clone, Debug)]
pub struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Image {
    /// The width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Encodes an image as a png.
    pub fn encode<W>(&self, writer: W) -> Result<(), png::EncodingError>
    where
        W: Write,
    {
        let mut encoder = png::Encoder::new(writer, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        writer.finish()
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

/// An error for [`Palette::from_bytes`].
#[derive(Debug, Display, Error)]
#[display("invalid palette len: {_0}")]
pub struct InvalidPaletteLength(#[error(not(source))] pub usize);
