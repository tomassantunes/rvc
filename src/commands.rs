use anyhow::anyhow;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path;
use walkdir::WalkDir;

// repo
// - .rvc
// -- commits
// --- v1
// ---- repo em v1
// --- v2
// -- commit_messages.txt

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".rvc").unwrap();
    fs::create_dir(".rvc/objects").unwrap(); // só porque o add já estava feito
    fs::create_dir(".rvc/commits").unwrap();
    fs::File::create(".rvc/commit_messages.txt").unwrap();

    Ok(())
}

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

    if hash_string.len() < 4 {
        return Err(anyhow::anyhow!("Hash is too short."));
    }

    let blob_path = format!(".rvc/objects/{}/{}", &hash_string[..2], &hash_string[2..]);

    let mut blob_file = fs::File::create(&blob_path)
        .map_err(|e| anyhow!("Failed to create file {}: {}", blob_path, e))?;

    blob_file
        .write_all(&contents)
        .map_err(|e| anyhow!("Failed to write blob file {}: {}", blob_path, e))?;

    Ok(())
}

// TODO: arranjar maneira de saber em que commit vou
// adicionar a message ao ficheiro de mensagens
pub fn commit(message: String) -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = current_dir.as_path();
    let dpath = path::Path::new(".rvc/commits");

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| {
        let file_name = e.file_name().to_string_lossy();
        println!("File name: {}", file_name);
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

        println!("Copied {}", &p.to_string_lossy());
    }

    Ok(())
}
