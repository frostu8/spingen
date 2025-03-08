//! The image encoding utilities.

use crate::doom::patch::{Palette, PALETTE_COLORS};
use crate::skin::{loader::Error, Skin, Sprite, SpriteAngle};
use crate::spray::Spray;

use std::io::Write;

use bevy_color::{Color, ColorToPacked, Srgba};

use derive_more::{Display, Error, From};

use wad::Name;

/// An image encoder for a skin.
#[derive(Debug)]
pub struct Encoder<'a> {
    skin_data: &'a Skin,
    palette: Palette,
}

impl<'a> Encoder<'a> {
    /// Creates a new `Encoder`.
    pub fn new(skin_data: &'a Skin) -> Encoder<'a> {
        Encoder {
            skin_data,
            palette: Palette::default(),
        }
    }

    /// Overrides the palette.
    ///
    /// By default, the palette is the Ring Racers palette.
    pub fn with_palette(self, palette: Palette) -> Encoder<'a> {
        Encoder { palette, ..self }
    }

    /// Applies a spray to the skin.
    pub fn with_spray(self, spray: &Spray) -> Encoder<'a> {
        Encoder {
            palette: spray.remap(&self.palette, self.skin_data.startcolor.into()),
            ..self
        }
    }

    /// Gets a single sprite of the skin by full, qualified name, and encodes
    /// it as a bitmap.
    pub fn sprite(&mut self, name: Name) -> Result<Image, Error> {
        let patch = self.skin_data.read_sprite(name)?;
        let image = sprite_to_image(&patch, &self.palette);

        Ok(image)
    }

    /// Gets a sprite index, and encodes it as a GIF.
    pub fn sprite_gif<W>(&mut self, writer: W, name: Name) -> Result<(), EncodeError>
    where
        W: Write,
    {
        let angles = &[
            SpriteAngle::FORWARD,
            SpriteAngle::RIGHT_FORWARD,
            SpriteAngle::RIGHT,
            SpriteAngle::RIGHT_BACKWARD,
            SpriteAngle::BACKWARD,
            SpriteAngle::LEFT_BACKWARD,
            SpriteAngle::LEFT,
            SpriteAngle::LEFT_FORWARD,
        ];

        if name.as_str().len() < 5 {
            return Err(Error::NotFound(name.to_string()).into());
        }

        let frame = name[4];
        let name = Name::from_bytes(&name[..4]).expect("valid subname");

        // get all patches
        let patches = self.skin_data.read_prefix(name.as_str())?;

        if patches.len() == 0 {
            return Err(Error::NotFound(name.to_string()).into());
        }

        // get all angles
        let mut angles = angles.into_iter().copied().filter_map(|angle| {
            patches
                .iter()
                .filter_map(|patch| {
                    if patch.identifier() == name {
                        patch.provides(frame, angle).map(|mirror| (patch, mirror))
                    } else {
                        None
                    }
                })
                .next()
        });

        // get first angle
        let Some((forward, mirror_forward)) = angles.next() else {
            return Err(Error::NotFound(name.to_string()).into());
        };

        // begin encoding a gif
        let mut palette = [0u8; PALETTE_COLORS * 3];
        for (i, color) in self.palette.iter().enumerate() {
            let color_bytes = color.to_srgba().to_u8_array();
            (&mut palette[i * 3..i * 3 + 3]).copy_from_slice(&color_bytes[..3]);
        }

        let mut gif = gif::Encoder::new(writer, forward.width, forward.height, &palette)?;
        gif.write_extension(gif::ExtensionData::Repetitions(gif::Repeat::Infinite))?;
        sprite_to_gif(&mut gif, forward, mirror_forward)?;

        // get other angles
        for (sprite, mirror) in angles {
            sprite_to_gif(&mut gif, sprite, mirror)?;
        }

        Ok(())
    }
}

fn sprite_to_gif<W>(
    gif: &mut gif::Encoder<W>,
    patch: &Sprite,
    mirror: bool,
) -> Result<(), gif::EncodingError>
where
    W: Write,
{
    // copy patch
    let mut buf = (0..(patch.width as usize * patch.height as usize))
        .map(|_| 0)
        .collect::<Vec<u8>>();

    for (i, palette_ix) in patch.data.iter().copied().enumerate() {
        let dest_i = if mirror {
            let col = i % patch.width as usize;
            let mirrored = patch.width as usize - col - 1;
            i - col + mirrored
        } else {
            i
        };

        buf[dest_i] = palette_ix.unwrap_or(255);
    }

    gif.write_frame(&gif::Frame {
        // TODO proper delay
        delay: 20,
        dispose: gif::DisposalMethod::Background,
        transparent: Some(255),
        buffer: buf.into(),
        width: patch.width,
        height: patch.height,
        ..Default::default()
    })?;

    Ok(())
}

fn sprite_to_image(patch: &Sprite, palette: &Palette) -> Image {
    let mut image = Image::new_fill(patch.width, patch.height, Color::WHITE);

    for (i, palette_ix) in patch.data.iter().enumerate() {
        let color_data = &mut image.data[i * 4..i * 4 + 4];

        if let Some(palette_ix) = palette_ix {
            palette.copy_color(*palette_ix as usize, color_data);
        } else {
            // encode transparent pixel
            palette.copy_color(255, color_data);
            color_data[3] = 0;
        }
    }

    image
}

/// A bitmap.
#[derive(Clone, Debug)]
pub struct Image {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

impl Image {
    /// Creates a new image, filled with a color.
    pub fn new_fill(width: u16, height: u16, color: Color) -> Image {
        let data = (0..(width as usize * height as usize))
            .flat_map(|_| {
                let srgb: Srgba = color.into();
                srgb.to_u8_array().into_iter()
            })
            .collect::<Vec<u8>>();

        Image {
            width,
            height,
            data,
        }
    }

    /// The width of the image.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// The height of the image.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Encodes an image as a png.
    pub fn to_png<W>(&self, writer: W) -> Result<(), EncodeError>
    where
        W: Write,
    {
        let mut encoder = png::Encoder::new(writer, self.width.into(), self.height.into());
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
        writer.finish().map_err(From::from)
    }
}

/// An error for encoding.
#[derive(Debug, Display, Error, From)]
pub enum EncodeError {
    Loader(Error),
    Gif(gif::EncodingError),
    Png(png::EncodingError),
    #[display("no angles to make gif")]
    NoAngles,
}

/*
/// Creates a spin gif from some skin data.
///
/// This takes in the skin, palette, and frame prefix.
pub fn gen_spin(data: &SkinData, palette: &Palette, ix: SpriteIndex) -> Result<String, Report> {
    let mut zip = ZipArchive::new(Cursor::new(data.data.clone())).wrap_err("invalid zip file")?;

    let mut angles = ix.angles();
    if angles.len() == 0 {
        return Err(Report::msg("sprite has no angles"));
    }

    // get first frame
    let path = data
        .path
        .join(angles.next().expect("should have one angle").as_str());
    let Some(entry_ix) = path.to_str().and_then(|e| zip.index_for_name(e)) else {
        // no entry found
        return Err(Report::msg(format!(
            "no valid thumbnail found foor {:?}",
            data.name
        )));
    };

    let mut entry = zip.by_index(entry_ix).wrap_err("invalid zip file")?;
    let mut scratch_buf = Vec::with_capacity(entry.size() as usize);
    entry
        .read_to_end(&mut scratch_buf)
        .wrap_err("invalid patch")?;

    // load patch
    let first_patch =
        Patch::read_with(Cursor::new(scratch_buf), palette).wrap_err("invalid patch")?;

    // for gif, create palette
    let global_palette = [0u8; PALETTE_COLORS * 3];
    for i in 0..palette.len() {
        let color = &mut global_palette[i * 3..];
        palette.copy_color(i, color);
    }

    // begin streaming gif
    let mut buf = Vec::new();
    let mut encoder = gif::Encoder::new(
        Cursor::new(buf),
        first_patch.image.width() as u16,
        first_patch.image.height() as u16,
        &global_palette,
    );
}
*/
