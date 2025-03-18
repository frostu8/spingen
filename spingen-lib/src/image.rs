//! The image encoding utilities.

use crate::doom::patch::{Palette, Patch, PALETTE_COLORS};
use crate::skin::Skin;
use crate::spray::Spray;
use crate::Error;

use std::io::Write;

use bevy_color::{Color, ColorToPacked, Srgba};

use derive_more::{Display, From};

use wad::Name;

use wasm_bindgen::prelude::*;

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
        self.sprite_with_options(writer, name, GifOptions::default())
    }

    /// Gets a single sprite of the skin by full, qualified name, and encodes
    /// it as a still PNG.
    pub fn sprite_with_options<W>(
        &mut self,
        writer: W,
        name: Name,
        options: GifOptions,
    ) -> Result<(), EncodeError>
    where
        W: Write,
    {
        let patch = self.skin_data.read(&name)?;
        patch_to_image_with_options(writer, &patch, &self.palette, options)
    }

    /// Gets a sprite index, and encodes it as an image.
    pub fn sprite_gif<W>(&mut self, writer: W, name: Name, frame: u8) -> Result<(), EncodeError>
    where
        W: Write,
    {
        self.sprite_gif_with_options(writer, name, frame, GifOptions::default())
    }

    /// Gets a sprite index, and encodes it as an image.
    pub fn sprite_gif_with_options<W>(
        &mut self,
        writer: W,
        name: Name,
        frame: u8,
        options: GifOptions,
    ) -> Result<(), EncodeError>
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
        if angles.len() == 0 {
            // create still png
            return self.sprite_with_options(writer, spr2.name, options);
        }

        let patch = self.skin_data.read(&spr2.name)?;
        let width = (patch.width as f32 * options.scale) as u16;
        let height = (patch.height as f32 * options.scale) as u16;

        // begin encoding a gif
        let mut palette = [0u8; PALETTE_COLORS * 3];
        for (i, color) in self.palette.iter().enumerate() {
            let color_bytes = color.to_srgba().to_u8_array();
            (&mut palette[i * 3..i * 3 + 3]).copy_from_slice(&color_bytes[..3]);
        }

        let mut gif = gif::Encoder::new(writer, width, height, &palette)?;
        gif.write_extension(gif::ExtensionData::Repetitions(gif::Repeat::Infinite))?;
        patch_to_gif_frame(
            &mut gif,
            &patch,
            GifOptions {
                mirror: spr2.mirror ^ options.mirror,
                ..options
            },
        )?;

        // get other angles
        for spr2 in angles.into_iter().rev() {
            let patch = self.skin_data.read(&spr2.name)?;
            patch_to_gif_frame(
                &mut gif,
                &patch,
                GifOptions {
                    mirror: spr2.mirror ^ options.mirror,
                    ..options
                },
            )?;
        }

        Ok(())
    }
}

/// GIF encode options.
#[derive(Clone, Debug, PartialEq)]
#[wasm_bindgen]
pub struct GifOptions {
    /// The factor to upscale by.
    pub scale: f32,
    /// The delay between each frame, in deciseconds.
    ///
    /// For PNGs, this is discarded.
    pub delay: u16,
    /// Whether to mirror across the X axis.
    pub mirror: bool,
}

#[wasm_bindgen]
impl GifOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GifOptions {
        GifOptions::default()
    }
}

impl Default for GifOptions {
    fn default() -> Self {
        GifOptions {
            scale: 1.,
            delay: 20,
            mirror: false,
        }
    }
}

fn patch_to_gif_frame<W>(
    gif: &mut gif::Encoder<W>,
    patch: &Patch,
    options: GifOptions,
) -> Result<(), gif::EncodingError>
where
    W: Write,
{
    // setup resample heights
    let width = (patch.width as f32 * options.scale) as usize;
    let height = (patch.height as f32 * options.scale) as usize;

    // copy patch
    let mut buf = (0..(width * height)).map(|_| 0).collect::<Vec<u8>>();

    for (i, dest) in buf.iter_mut().enumerate() {
        // sample by nearest neighbor
        let dest_x = i % width;
        let dest_y = i / width;

        let src_x = (dest_x as f32 / options.scale) as usize;
        let src_y = (dest_y as f32 / options.scale) as usize;

        // saturate
        //let src_x = min(src_x, patch.width as usize - 1);
        //let src_y = min(src_y, patch.height as usize - 1);

        let src_x = if options.mirror {
            patch.width as usize - src_x - 1
        } else {
            src_x
        };

        *dest = patch.data[src_y * patch.width as usize + src_x].unwrap_or(255);
    }

    gif.write_frame(&gif::Frame {
        delay: options.delay,
        dispose: gif::DisposalMethod::Background,
        transparent: Some(255),
        buffer: buf.into(),
        width: width as u16,
        height: height as u16,
        ..Default::default()
    })?;

    Ok(())
}

/// Converts a patch to a still PNG.
pub fn patch_to_image<W>(writer: W, patch: &Patch, palette: &Palette) -> Result<(), EncodeError>
where
    W: Write,
{
    patch_to_image_with_options(writer, patch, palette, GifOptions::default())
}

/// Converts a patch to a still PNG.
pub fn patch_to_image_with_options<W>(
    writer: W,
    patch: &Patch,
    palette: &Palette,
    options: GifOptions,
) -> Result<(), EncodeError>
where
    W: Write,
{
    // setup resample heights
    let width = (patch.width as f32 * options.scale) as usize;
    let height = (patch.height as f32 * options.scale) as usize;

    let mut data = (0..width * height)
        .flat_map(|_| {
            let srgb: Srgba = Color::WHITE.into();
            srgb.to_u8_array().into_iter()
        })
        .collect::<Vec<u8>>();

    for i in 0..width * height {
        // sample by nearest neighbor
        let dest_x = i % width;
        let dest_y = i / width;

        let src_x = (dest_x as f32 / options.scale) as usize;
        let src_y = (dest_y as f32 / options.scale) as usize;

        // saturate
        //let src_x = min(src_x, patch.width as usize - 1);
        //let src_y = min(src_y, patch.height as usize - 1);

        let src_x = if options.mirror {
            patch.width as usize - src_x - 1
        } else {
            src_x
        };

        let palette_ix = patch.data[src_y * patch.width as usize + src_x];
        let color_data = &mut data[i * 4..i * 4 + 4];

        if let Some(palette_ix) = palette_ix {
            palette.copy_color(palette_ix as usize, color_data);
        } else {
            // encode transparent pixel
            palette.copy_color(255, color_data);
            color_data[3] = 0;
        }
    }

    let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
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
