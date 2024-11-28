use std::env::args;

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
    pub fn parse() -> Result<Args, ProgramError> {

        enum FlagSet {
            SetText,
            SetSize,
            Nop
        }

        let mut result = Self::default();

        let mut flag_to_set = FlagSet::Nop;
        for arg in args() {
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

        Ok(result)
    }
}

