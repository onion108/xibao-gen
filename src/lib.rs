#![allow(unused)]
use std::{fs::OpenOptions, io::Write};

use argparse::Args;
use error::ProgramError;
use previewer::run_preview;
use render::{create_surface, render_image};
use resource::get_res_path;
use skia_safe::{surfaces, Canvas, Color, Color4f, Data, Font, FontMgr, FontStyle, Image, ImageInfo, Paint, PaintStyle, Path, Point, Size, Surface, Typeface};

pub mod error;
pub mod resource;
pub mod argparse;
pub mod render;
pub mod previewer;

// From the width and height of the image.
const WIDTH: i32 = 980;
const HEIGHT: i32 = 735;

pub fn program_entry() -> Result<(), ProgramError> {
    let args = Args::parse()?;

    let img = render_image(WIDTH, HEIGHT, &args)?;
    let imgdata = img.encode(None, skia_safe::EncodedImageFormat::PNG, None).ok_or(ProgramError::SkiaNoImage)?;

    if args.preview {
        run_preview(imgdata, WIDTH, HEIGHT)?;
    } else {
        let mut img_file = OpenOptions::new().write(true).create(true).open("output.png")?;
        img_file.write(imgdata.as_bytes())?;
    }
    Ok(())
}

