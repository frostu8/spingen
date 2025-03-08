//! The image encoding utilities.

use crate::doom::patch::{Palette, Patch, PALETTE_COLORS};
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
    /// it as a still PNG.
    pub fn sprite<W>(&mut self, writer: W, name: Name) -> Result<(), EncodeError>
    where
        W: Write,
    {
        let patch = self.skin_data.read(name)?;

        let mut data = (0..(patch.width as usize * patch.height as usize))
            .flat_map(|_| {
                let srgb: Srgba = Color::WHITE.into();
                srgb.to_u8_array().into_iter()
            })
            .collect::<Vec<u8>>();

        for (i, palette_ix) in patch.data.iter().enumerate() {
            let color_data = &mut data[i * 4..i * 4 + 4];

            if let Some(palette_ix) = palette_ix {
                self.palette.copy_color(*palette_ix as usize, color_data);
            } else {
                // encode transparent pixel
                self.palette.copy_color(255, color_data);
                color_data[3] = 0;
            }
        }

        let mut encoder = png::Encoder::new(writer, patch.width.into(), patch.height.into());
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
        writer.write_image_data(&data)?;
        writer.finish().map_err(From::from)
    }

    /// Gets a sprite index, and encodes it as an image.
    pub fn sprite_gif<W>(&mut self, writer: W, name: Name, frame: u8) -> Result<(), EncodeError>
    where
        W: Write,
    {
        if name.as_str().len() < 4 {
            return Err(Error::NotFound(name.to_string()).into());
        }

        // get all patches
        let mut angles = self.skin_data.iter_angles(&name, frame).collect::<Vec<_>>();
        if angles.len() == 0 {
            return Err(Error::NotFound(name.to_string()).into());
        }
        angles.sort_by(|a, b| a.index.angle.cmp(&b.index.angle).reverse());

        // get first angle
        let Some(spr2) = angles.pop() else {
            return Err(Error::NotFound(name.to_string()).into());
        };
        let patch = self.skin_data.read(spr2.name)?;

        // begin encoding a gif
        let mut palette = [0u8; PALETTE_COLORS * 3];
        for (i, color) in self.palette.iter().enumerate() {
            let color_bytes = color.to_srgba().to_u8_array();
            (&mut palette[i * 3..i * 3 + 3]).copy_from_slice(&color_bytes[..3]);
        }

        let mut gif = gif::Encoder::new(writer, patch.width, patch.height, &palette)?;
        gif.write_extension(gif::ExtensionData::Repetitions(gif::Repeat::Infinite))?;
        patch_to_gif_frame(&mut gif, &patch, spr2.mirror)?;

        // get other angles
        for spr2 in angles.into_iter().rev() {
            let patch = self.skin_data.read(spr2.name)?;
            patch_to_gif_frame(&mut gif, &patch, spr2.mirror)?;
        }

        Ok(())
    }
}

fn patch_to_gif_frame<W>(
    gif: &mut gif::Encoder<W>,
    patch: &Patch,
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

/// An error for encoding.
#[derive(Debug, Display, Error, From)]
pub enum EncodeError {
    Loader(Error),
    Gif(gif::EncodingError),
    Png(png::EncodingError),
    #[display("no angles to make gif")]
    NoAngles,
}

impl EncodeError {
    pub fn not_found(&self) -> bool {
        if let EncodeError::Loader(err) = self {
            err.not_found()
        } else {
            false
        }
    }
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
