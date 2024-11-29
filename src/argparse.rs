use std::{env::args, process::exit};

use crate::error::ProgramError;

#[derive(Debug, Clone)]
pub struct Args {
    pub preview: bool,
    pub text: String,
    pub size: i32,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            preview: false,
            text: "".into(),
            size: 48
        }
    }
}

impl Args {
    pub fn show_help(exe_name: &str) {
        const HELP_MESSAGE: &'static str = r#"
        Usage:
        %$<PROG_NAME>$% [--preview] [--text|-t text] [--size|-s font_size] [--help] <text>
        
        Arguments:
            --preview  Show a window to preview the image instead of writing to file.
        -t, --text     The text to display in the generated image.
        -s, --size     The font size, defaults to 48.
            --help     Display this message.
            <text>     Same as using -t or --text.

        If duplicate arguments are read, then the latest one will be applied, others will be ignored.
        "#;

        eprintln!("{}", 
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
            Nop
        }

        let mut result = Self::default();
        let mut args = args();
        let exe_name = args.next().unwrap();

        let mut flag_to_set = FlagSet::Nop;
        let mut empty = true;
        for arg in args {
            empty = false;
            match (arg.as_str(), &flag_to_set) {
                ("--preview" | "-p", FlagSet::Nop) => {
                    result.preview = true
                },
                ("--text" | "-t", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetText
                }
                ("--size" | "-s", FlagSet::Nop) => {
                    flag_to_set = FlagSet::SetSize
                }
                ("--help", FlagSet::Nop) => {
                    Self::show_help(&exe_name);
                    exit(0);
                }
                (s, FlagSet::Nop | FlagSet::SetText) => {
                    result.text = s.into();
                    flag_to_set = FlagSet::Nop;
                }
                (s, FlagSet::SetSize) => {
                    result.size = s.parse::<i32>()?;
                    flag_to_set = FlagSet::Nop;
                }
            }
        }

        if empty {
            Self::show_help(&exe_name);
            exit(1);
        }

        Ok(result)
    }
}

