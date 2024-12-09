use std::{env::args, fs::OpenOptions, io::Read, process::exit};

use crate::error::ProgramError;

#[derive(Debug, Clone)]
pub struct Args {
    pub preview: bool,
    pub text: String,
    pub size: i32,
    pub out: String,
    pub font: Option<String>,
    pub custom_bg: Option<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            preview: false,
            text: "".into(),
            size: 48,
            out: "output.png".into(),
            font: None,
            custom_bg: None,
        }
    }
}

impl Args {
    pub fn show_help(exe_name: &str) {
        const HELP_MESSAGE: &'static str = r#"
        Usage:
        %$<PROG_NAME>$% [--preview] [--text|-t text] [--size|-s font_size] [--from-file|-i file] [--to-file|-o file] [--font|-f font] [--help] <text>
        
        Arguments:
            --preview    Show a window to preview the image instead of writing to file.
        -t, --text       The text to display in the generated image.
        -i, --from-file  Read text from file and use it as input.
        -o, --to-file    Specify output file name. Defaults to "output.png". If the argument doesn't end with .png the extension will be added.
        -s, --size       The font size, defaults to 48.
        -b, --background Path to the custom background image.
        -f, --font       Specify the font family name used by the image.
            --help       Display this message.
            <text>       Same as using -t or --text.

        If duplicate arguments are read, then the latest one will be applied, others will be ignored.
        "#;

        eprintln!(
            "{}",
            HELP_MESSAGE
                .trim_end()
                .replace("%$<PROG_NAME>$%", exe_name)
                .split("\n")
                .map(|line| line.replacen("        ", "", 1))
                .fold(String::new(), |a, b| a + &b + "\n")
        );
    }

    pub fn parse() -> Result<Args, ProgramError> {
        enum FlagSet {
            SetText,
            SetSize,
            SetInput,
            SetOutput,
            SetBackground,
            SetFont,
            Nop,
        }

        let mut result = Self::default();
        let mut args = args();
        let exe_name = args.next().unwrap();

        let mut flag_to_set = FlagSet::Nop;
        let mut empty = true;
        let mut text_specified = false;
        for arg in args {
            empty = false;
            match (arg.as_str(), &flag_to_set) {
                ("--preview" | "-p", FlagSet::Nop) => {
                    result.preview = true;
                }
                ("--text" | "-t", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetText;
                }
                ("--from-file" | "-i", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetInput;
                }
                ("--to-file" | "-o", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetOutput;
                }
                ("--size" | "-s", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetSize;
                }
                ("--background" | "-b", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetBackground;
                }
                ("--font" | "-f", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetFont;
                }
                ("--help", FlagSet::Nop) => {
                    Self::show_help(&exe_name);
                    exit(0);
                }
                (s, FlagSet::Nop | FlagSet::SetText) => {
                    result.text = s.into();
                    text_specified = true;
                    flag_to_set = FlagSet::Nop;
                }
                (s, FlagSet::SetSize) => {
                    result.size = s.parse::<i32>()?;
                    flag_to_set = FlagSet::Nop;
                }
                (path, FlagSet::SetInput) => {
                    // Read file from the path and set it as content.
                    let mut file = OpenOptions::new().read(true).open(path)?;
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    result.text = content;
                    text_specified = true;
                    flag_to_set = FlagSet::Nop;
                }
                (path, FlagSet::SetOutput) => {
                    result.out = if path.ends_with(".png") {
                        path.into()
                    } else {
                        format!("{}.png", path)
                    };
                    flag_to_set = FlagSet::Nop;
                }
                (font, FlagSet::SetFont) => {
                    result.font = Some(font.into());
                    flag_to_set = FlagSet::Nop;
                }
                (path, FlagSet::SetBackground) => {
                    result.custom_bg = Some(path.into());
                    flag_to_set = FlagSet::Nop;
                }
            }
        }

        if empty {
            Self::show_help(&exe_name);
            exit(1);
        }

        if !text_specified {
            return Err(ProgramError::ArgParseMissingContent);
        }

        match flag_to_set {
            FlagSet::Nop => Ok(result),

            // If the flag_to_set isn't `FlagSet::Nop`, meaning some argument are not given for a
            // flag.
            FlagSet::SetOutput => Err(ProgramError::ArgParseMissingFlagValue(
                "--to-file/-o".into(),
            )),
            FlagSet::SetInput => Err(ProgramError::ArgParseMissingFlagValue(
                "--from-file/-i".into(),
            )),
            FlagSet::SetText => Err(ProgramError::ArgParseMissingFlagValue("--text/-t".into())),
            FlagSet::SetSize => Err(ProgramError::ArgParseMissingFlagValue("--size/-s".into())),
            FlagSet::SetBackground => Err(ProgramError::ArgParseMissingFlagValue(
                "--background/-b".into(),
            )),
            FlagSet::SetFont => Err(ProgramError::ArgParseMissingFlagValue(
                "--background/-b".into(),
            )),
        }
    }
}
