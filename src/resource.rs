use std::{env::current_exe, fs::exists};

use crate::error::ProgramError;

/// Find resource by relative path.
/// It will find resource in the following position:
/// - `$PWD/resource/`
/// - `$PWD/share/`
/// - `@executable_path/../`
/// - `@executable_path/../../share/<executable_name>`
pub fn get_res_path(path: &str) -> Result<String, ProgramError> {
    // Special case for absolute path.
    if path.starts_with("/") {
        return exists(path)
            .map_err(|err| ProgramError::from(err))
            .and_then(|x| {
                if x {
                    Ok(path.to_string())
                } else {
                    Err(ProgramError::ResourceNotFound { path: path.into() })
                }
            });
    }

    let possible_path0 = String::from("resource/") + path.as_ref();
    if exists(&possible_path0)? {
        return Ok(possible_path0);
    }

    let possible_path1 = String::from("share/") + path.as_ref();
    if exists(&possible_path1)? {
        return Ok(possible_path1);
    }

    if let Ok(mut path_buf) = current_exe() {
        let exe_name = path_buf
            .file_name()
            .unwrap()
            .to_str()
            .ok_or(ProgramError::EncodingError)?
            .to_string();
        path_buf.pop();
        let possible_path2 = path_buf.join(path);
        if exists(&possible_path2)? {
            return Ok(possible_path2
                .to_str()
                .ok_or(ProgramError::EncodingError)?
                .into());
        }

        let possible_path3 = path_buf.join("../share").join(&exe_name).join(path);
        if exists(&possible_path3)? {
            return Ok(possible_path3
                .to_str()
                .ok_or(ProgramError::EncodingError)?
                .into());
        }
    }

    return Err(ProgramError::ResourceNotFound { path: path.into() });
}
