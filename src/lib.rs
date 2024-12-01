use std::{fs::OpenOptions, io::Write};

use argparse::Args;
use error::ProgramError;
use previewer::run_preview;
use render::render_image;

pub mod argparse;
pub mod error;
pub mod previewer;
pub mod render;
pub mod resource;

pub fn program_entry() -> Result<(), ProgramError> {
    let args = Args::parse()?;

    let img = render_image(&args)?;
    let imgdata = img
        .encode(None, skia_safe::EncodedImageFormat::PNG, None)
        .ok_or(ProgramError::SkiaNoImage)?;

    if args.preview {
        run_preview(imgdata, img.width(), img.height())?;
    } else {
        let mut img_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&args.out)?;
        img_file.write(imgdata.as_bytes())?;
    }
    Ok(())
}
