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
