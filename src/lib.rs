#![allow(unused)]
use std::{fs::OpenOptions, io::Write};

use argparse::Args;
use error::ProgramError;
use resource::get_res_path;
use skia_safe::{surfaces, Canvas, Color, Color4f, Data, Font, FontMgr, FontStyle, Image, ImageInfo, Paint, PaintStyle, Path, Point, Size, Surface, Typeface};

pub mod error;
pub mod resource;
pub mod argparse;

// From the width and height of the image.
const WIDTH: i32 = 980;
const HEIGHT: i32 = 735;

pub fn program_entry() -> Result<(), ProgramError> {
    let arg = Args::parse()?;

    let mut surface = surfaces::raster(
        &ImageInfo::new((WIDTH, HEIGHT), skia_safe::ColorType::RGBA8888, skia_safe::AlphaType::Opaque, None),
        None,
        None
    ).ok_or(ProgramError::SkiaNoSurface)?;
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::BLACK);

    let font_mgr = FontMgr::new();
    let default_typeface = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();

    let bg_img = Image::from_encoded(Data::from_filename(get_res_path("xb_bg.png")?).unwrap()).ok_or(ProgramError::SkiaNoImage)?;

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    canvas.draw_image(&bg_img, (0, 0), Some(&paint));

    let font = &Font::from_typeface(default_typeface, arg.size as f32);
    let shaper = skia_safe::Shaper::new(FontMgr::new());
    if let Some((blob, _)) =
    shaper.shape_text_blob(&arg.text, font, true, 10000.0, Point::default())
    {
        let bounds = blob.bounds();
        let text_width = bounds.right - bounds.left;
        let text_height = bounds.bottom - bounds.top;
        println!("({text_width}, {text_height})");
        let text_x = (WIDTH as f32 - text_width) / 2.0;
        let text_y = (HEIGHT as f32 - text_height) / 2.0;
        canvas.draw_text_blob(&blob, (text_x, text_y), &paint);
    }

    let img = surface.image_snapshot();
    let imgdata = img.encode(None, skia_safe::EncodedImageFormat::PNG, None).ok_or(ProgramError::SkiaNoImage)?;
    let mut img_file = OpenOptions::new().write(true).create(true).open("output.png")?;
    img_file.write(imgdata.as_bytes())?;
    Ok(())
}

