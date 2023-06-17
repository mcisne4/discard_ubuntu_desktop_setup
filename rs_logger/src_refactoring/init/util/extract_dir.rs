use std::path::PathBuf;

pub fn extract_dir(path: &PathBuf) -> Result<PathBuf, String> {
    let path_original = path.clone();

    let mut path = path.clone();

    if path.is_file() {
        path.pop();
    }

    if !path.is_dir() {
        return Err(format!(
            "ERROR: A directory could not be extracted from the given path:\n  {}",
            path_original.display(),
        ));
    }

    Ok(path)
}
