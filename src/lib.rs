use std::fs;
use std::path::Path;

use regex::Regex;
use serde_json::Value;
use std::io::{Error, ErrorKind};

pub fn write_files(path: &Path, value: &Value) -> Result<(), Error> {
    _write_files(Regex::new(r"^[\w\-]+$").unwrap(), path, value)
}

#[inline]
fn _write_files(key_re: Regex, path: &Path, value: &Value) -> Result<(), Error> {
    match value {
        Value::Array(array) => {
            for (i, value) in array.iter().enumerate() {
                _write_files(key_re, path.join(i.to_string()).as_path(), value)?;
            }
        }
        Value::Object(map) => {
            for (key, value) in map.iter() {
                // Validation to avoid tree traversal or other issues
                if !key_re.is_match(key) {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        r"Keys must match: ^[\w\-]+$",
                    ));
                }

                _write_files(key_re, &path.join(key).as_path(), value)?;
            }
        }
        _ => {
            if let Some(parent) = path.parent() {
                if !parent.is_dir() {
                    fs::create_dir_all(parent)?;
                }
            }

            fs::write(path, value.to_string())?;
        }
    }

    Ok(())
}
