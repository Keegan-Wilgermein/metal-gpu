//! Path.rs
//! 
//! Functions for path stuff
//! 

use std::{fs::File, io::Read, path::PathBuf};

use crate::modules::{into_ext::IntoExtend};

/// Reads a file and returns its contents as a `String`
pub(crate) fn file_as_str(
    path: impl Into<PathBuf>,
) -> Option<String> {
    let path = path.into();

    // Check file validity
    if !path.exists() { return None; }
    if !path.is_file() { return None; }
    if !path.extension().is_some_and(|osstr| {
        osstr.to_str().is_some_and(|s| {
            s.eq("metal") || s.eq(".metal")
        })
    }) { return None; }

    let mut file = File::open(path).ok()?;
    let file_size_bytes = file.metadata().ok()?.len();
    let mut buffer = String::with_capacity(file_size_bytes.into_idx());
    file.read_to_string(&mut buffer).ok()?;

    Some(buffer)
}
