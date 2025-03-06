//! The image-generation utilities.
//!
//! These will generate the images and return urls so they can be embedded onto
//! the page.

use crate::doom::patch::{Palette, Patch, PALETTE_COLORS};
use crate::skin::{SkinData, SpriteIndex};

use std::io::{Cursor, Read as _};

use eyre::{Report, WrapErr};

use zip::ZipArchive;

use gloo::file::Blob;

use web_sys::Url;

/// Creates a thumbnail from some skin data.
///
/// This searches for `STINA2` for asymmetrical skins, and `STINA2A8` for
/// symmetrical skins.
pub fn gen_thumbnail(data: &SkinData, palette: &Palette) -> Result<String, Report> {
    let mut zip = ZipArchive::new(Cursor::new(data.data.clone())).wrap_err("invalid zip file")?;

    let path = data.path.join("STINA2");
    let asym_path = data.path.join("STINA2A8");

    let Some(entry_ix) = path
        .to_str()
        .and_then(|e| zip.index_for_name(e))
        .or(asym_path.to_str().and_then(|e| zip.index_for_name(e)))
    else {
        // no entry found
        return Err(Report::msg(format!(
            "no valid thumbnail found foor {:?}",
            data.name
        )));
    };

    let mut entry = zip.by_index(entry_ix).wrap_err("invalid zip file")?;
    let mut buf = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut buf).wrap_err("invalid patch")?;

    // load patch
    let patch = Patch::read_with(Cursor::new(buf), palette).wrap_err("invalid patch")?;
    // encode image from patch
    let mut buf = Vec::new();
    patch
        .image
        .encode(Cursor::new(&mut buf))
        .wrap_err("failed to encode patch")?;

    // create blob
    let blob = Blob::new_with_options(&buf[..], Some("image/png"));

    Ok(Url::create_object_url_with_blob(blob.as_ref()).expect("valid blob url"))
}

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
