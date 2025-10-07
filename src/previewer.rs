use sdl3::{
    event::{Event, WindowEvent},
    image::{LoadTexture},
    keyboard::Keycode,
};
use skia_safe::Data;

use crate::error::ProgramError;

pub fn run_preview(png_image: Data, width: i32, height: i32) -> Result<(), ProgramError> {
    let sdl_ctx = sdl3::init()?;
    //let _ = sdl3::image::init(InitFlag::PNG).map_err(|x| ProgramError::SDLError(x.to_string()))?;
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem
        .window("Preview", width as u32, height as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture_bytes(&png_image)?;

    canvas
        .copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_ctx
            .event_pump()?
            .poll_iter()
        {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'mainloop;
                }
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Moved(_, _) => {
                            canvas
                                .copy(&texture, None, None)?;
                            canvas.present();
                        }
                        WindowEvent::Resized(_, _) => {
                            canvas
                                .copy(&texture, None, None)?;
                            canvas.present();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
