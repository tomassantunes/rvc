use anyhow::anyhow;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path;
use walkdir::WalkDir;

use crate::utils;

// repo
// - .rvc
// -- commits
// --- v1
// ---- repo em v1
// --- v2
// -- commit_messages.txt

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".rvc").unwrap();
    fs::create_dir(".rvc/objects").unwrap();
    fs::create_dir(".rvc/commits").unwrap();
    fs::File::create(".rvc/commit_messages.txt").unwrap();
    fs::File::create(".rvc/index").unwrap(); // file that contains the staged changes

    Ok(())
}

pub fn add(_path: String) -> anyhow::Result<()> {
    let path = path::Path::new(&_path);
    let index_path = path::Path::new(".rvc/index");
    let mut index_file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(index_path).expect("failed to open index file");

    if path.is_dir() {
        writeln!(&mut index_file, "{}|dir", path.display()).expect("failed to write to index file");

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                add(path.display().to_string())?;
            } else {
                add_file_to_index(&path, &mut index_file)?;
            }
        }
    } else {
        add_file_to_index(&path, &mut index_file)?;
    }

    Ok(())
}

pub fn add_file_to_index(path: &path::Path, index_file: &mut fs::File) -> anyhow::Result<()> {
    let mut file = fs::File::open(path)?;
    let contents = utils::hash_file_contents(&mut file)?;

    writeln!(index_file, "{}|{}", path.display(), contents).expect("failed to write to index file");

    Ok(())
}

/*
pub fn add(path: String) -> anyhow::Result<()> {
let p = path::Path::new(&path);
let mut file = fs::File::open(p).map_err(|e| anyhow!("Failed to open file {}: {}", path, e))?;

let mut contents = Vec::new();
file.read_to_end(&mut contents)
.map_err(|e| anyhow!("Failed to read file {}: {}", path, e))?;

let mut hasher = Sha256::new();
hasher.update(&contents);
let hash = hasher.finalize();
let hash_string = format!("{:x}", hash);

let blob_path = format!(".rvc/objects/{}/{}", &hash_string[..2], &hash_string[2..]);

fs::create_dir(format!(".rvc/objects/{}", &hash_string[..2])).expect("failed to create object directory");

let mut blob_file = fs::File::create(&blob_path)
.map_err(|e| anyhow!("Failed to create file {}: {}", blob_path, e))?;

blob_file
.write_all(&contents)
.map_err(|e| anyhow!("Failed to write blob file {}: {}", blob_path, e))?;

Ok(())
}
*/

pub fn commit(message: String) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = current_dir.as_path();
    let mut dpath = path::Path::new("./.rvc/commits/");

    let child_dirs: u32 = utils::count_children_dir(dpath)?;

    let new_dpath: String = format!("{}v{}", dpath.to_string_lossy(), child_dirs);

    dpath = path::Path::new(&new_dpath);
    fs::create_dir(&dpath)?;

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| {
        let file_name = e.file_name().to_string_lossy();
        !file_name.starts_with(".") && !file_name.starts_with("_")
    }) {
        let entry = entry?;
        let p = entry.path();

        let relative_p = p.strip_prefix(path)?;

        let target_p = dpath.join(relative_p);

        if p.is_dir() {
            fs::create_dir_all(&target_p)?;
        } else {
            if let Some(parent) = target_p.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(p, &target_p)?;
        }
    }

    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./.rvc/commit_messages.txt").expect("cannot open file")
        .write(format!("[v{}] {}\n", child_dirs, message).as_bytes()).expect("commit message write failed");

    Ok(())
}
