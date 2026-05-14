use std::num::ParseIntError;

use sdl3::{video::WindowBuildError, IntegerOrSdlError};
use thiserror::Error;

/// A wrapper for all kinds of errors that may happen in the program.
#[derive(Error, Debug)]
pub enum ProgramError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("cannot find resource file")]
    ResourceNotFound { path: String },

    #[error("failed at utf8 validity check, maybe using some evil non-utf8 encoding 😈")]
    EncodingError,

    #[error(transparent)]
    ArgParseError(#[from] ParseIntError),

    #[error("please specify argument for flag {0}")]
    ArgParseMissingFlagValue(String),

    #[error("please specify content text")]
    ArgParseMissingContent,

    #[error("cannot initialize skia canvas")]
    SkiaNoCanvas,

    #[error("cannot initialize skia surface")]
    SkiaNoSurface,

    #[error("cannot read or generate image data")]
    SkiaNoImage,

    #[error("SDL2 pooped itself: {0}")]
    SDLError(#[from] sdl3::Error),

    #[error(transparent)]
    SDLIntegerOrError(#[from] IntegerOrSdlError),

    #[error(transparent)]
    SDLWindowBuildError(#[from] WindowBuildError),

    #[error("cannot find font family {0}")]
    NoFontFamilyName(String),

    /// The motherfucker clipboard-rs doesn't specify exact Error type what the fuck are you guys
    /// doing now I have to embed this silly stupid type into my Error type I hate you guys really
    /// really much I can't even express my hatred through words properly oh my god I hate this
    /// thing so much.
    ///
    /// I'll never want to see the stupid `Box<dyn std::error::Error + Send + Sync + 'static>` any more.
    /// **FUCK IT**.
    #[error(transparent)]
    ClipboardIrresponsibleError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}
