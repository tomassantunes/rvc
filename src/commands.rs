use std::fs;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path;

use crate::utils;

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".rvc").unwrap();
    fs::create_dir(".rvc/objects").unwrap();
    fs::create_dir(".rvc/commits").unwrap();
    fs::File::create(".rvc/index").unwrap(); // file that contains the staged changes
    fs::File::create(".rvc/HEAD").unwrap(); // file that contains commit references

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

pub fn commit(message: String) -> anyhow::Result<()> {
    let current_time = chrono::offset::Utc::now();
    let user_name = whoami::username();
    let user_real_name = whoami::realname();
    let commit_message = format!("{}\n{} - {}\n{}", message, user_name, user_real_name, current_time);
    let commit_message_hash = utils::hash_string(commit_message.clone()).expect("failed to hash the commit message");
    let commit_path = format!(".rvc/objects/{}/{}", &commit_message_hash[..2], &commit_message_hash[2..]);
    fs::create_dir_all(&commit_path).expect("failed to create commit directories");

    let index_path = path::Path::new(".rvc/index");
    let index_file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(index_path).expect("failed to open index file");
    let reader = BufReader::new(index_file);

    for line in reader.lines() {
        let line = line.expect("failed to get line");
        let parts: Vec<&str> = line.split("|").collect(); 
        let path = path::Path::new(parts[0]);

        if path.is_dir() {
            continue;
        }
        // what should I do with the hash?
        let mut file = fs::File::open(path).expect("failed to open file");
        let compressed_contents = utils::compress_file(&mut file).expect("failed to compress file");

        let file_path = format!("{}/{}", commit_path, path.to_path_buf().display());
        if let Some(parent_dir) = path::Path::new(&file_path).parent() {
            fs::create_dir_all(parent_dir).expect("failed to create parent dir");
            println!("{}", parent_dir.display());
        }

        let mut file = fs::File::create(file_path).expect("failed to create file for object");
        file.write_all(&compressed_contents).expect("failed to write compressed content to file");
    }

    let commit_message_path = format!("{}/commit_message.txt", commit_path);
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(commit_message_path).expect("cannot open file")
        .write_all(&commit_message.as_bytes()).expect("commit message write failed");

    Ok(())
}
