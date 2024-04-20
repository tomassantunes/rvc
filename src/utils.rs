use sha2::Digest;
use std::{fs, io::Read, path::{Path, PathBuf}};
use walkdir::WalkDir;

pub fn count_children_dir(path: &Path) -> anyhow::Result<u32> {
    let mut count: u32 = 0;

    for d in WalkDir::new(path).max_depth(1).into_iter() {
        let d = d?;

        if d.into_path().is_dir() {
            count += 1;
        }
    }

    return Ok(count);
}

pub fn get_files_from_dir(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();

    for d in WalkDir::new(path).into_iter() {
        let d = d?;

        result.push(d.into_path());
    }

    return Ok(result);
}

pub fn hash_file_contents(mut file: &fs::File) -> anyhow::Result<String> {
    let mut hasher = sha2::Sha256::new();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    hasher.update(&buffer);
    let hash_result = hasher.finalize();

    Ok(format!("{:x}", hash_result))
}
