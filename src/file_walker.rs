use std::{
    fmt::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn get_all_images_in_path(pattern: &str) -> Result<Vec<PathBuf>, Error> {
    let results: Vec<PathBuf> = walkdir::WalkDir::new(pattern)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_file_image(entry.path()))
        .map(|entry| fs::canonicalize(entry.path()).expect("File must exist!"))
        .collect();
    Ok(results)
}

fn is_file_image(path: &Path) -> bool {
    match imghdr::from_file(
        fs::canonicalize(path).expect("File have been removed during the process!"),
    ) {
        Ok(Some(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_files() {
        let files = get_all_images_in_path("./tests").expect("Must be ok!");
        assert_eq!(files.len(), 4);
    }
}
