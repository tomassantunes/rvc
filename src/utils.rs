use std::path::Path;
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

pub fn get_files_from_dir(path: &Path) -> anyhow::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    for d in WalkDir::new(path).into_iter() {
        let d = d?;

        let x: String = d.into_path().into_os_string().into_string().unwrap();
        result.push(x);
    }

    return Ok(result);
}
