use skia_safe::{surfaces, Color, Color4f, Data, Font, FontMgr, FontStyle, Image, ImageInfo, Paint, Point, Surface};

use crate::{argparse::Args, error::ProgramError, resource::get_res_path};

pub fn create_surface(width: i32, height: i32) -> Result<Surface, ProgramError> {
    surfaces::raster(
        &ImageInfo::new((width, height), skia_safe::ColorType::RGBA8888, skia_safe::AlphaType::Opaque, None),
        None,
        None
    ).ok_or(ProgramError::SkiaNoSurface)
}

pub fn render_image(width: i32, height: i32, args: &Args) -> Result<Image, ProgramError> {
    let mut surface = create_surface(width, height)?;
    let canvas = surface.canvas();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::BLACK);

    let mut debug_painter = Paint::default();
    debug_painter.set_color(Color::TRANSPARENT);
    debug_painter.set_style(skia_safe::PaintStyle::Stroke);
    debug_painter.set_stroke_width(4.0);

    let font_mgr = FontMgr::new();
    let default_typeface = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();

    let bg_img = Image::from_encoded(Data::from_filename(get_res_path("xb_bg.png")?).unwrap()).ok_or(ProgramError::SkiaNoImage)?;

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));
    canvas.draw_image(&bg_img, (0, 0), Some(&paint));

    // Render text on the image
    // TODO: Handle linefeeds.
    let font = &Font::from_typeface(default_typeface, args.size as f32);
    let shaper = skia_safe::Shaper::new(FontMgr::new());
    if let Some((blob, _)) =
    shaper.shape_text_blob(&args.text, font, true, 10000.0, Point::default())
    {
        let bounds = blob.bounds();
        println!("{:?}", bounds);
        let text_width = bounds.right - bounds.left;
        let text_height = bounds.bottom - bounds.top;
        let text_x = (width as f32 - text_width) / 2.0 - bounds.left;
        let text_y = (height as f32 - text_height) / 2.0 - bounds.top;
        canvas.draw_text_blob(&blob, (text_x, text_y), &paint);
        
        let boundary = bounds.clone().with_offset((text_x, text_y));
        canvas.draw_rect(boundary, &debug_painter);
        canvas.draw_point((text_x, text_y), &debug_painter);
    }

    Ok(surface.image_snapshot())
}

