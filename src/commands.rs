use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::fs::OpenOptions;
use std::io::Write;
use std::path;

use crate::utils;

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".rvc").unwrap();
    fs::create_dir(".rvc/objects").unwrap();
    fs::create_dir(".rvc/messages").unwrap();
    fs::File::create(".rvc/index").unwrap(); // file that contains the staged changes
    fs::File::create(".rvc/commits").unwrap(); // file that contains commit references
    fs::File::create(".rvc/HEAD").unwrap(); // file that contains the current commit id

    println!("Initialized the repositoruy.");
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

    println!("Added changes successfully.");
    Ok(())
}

fn add_file_to_index(path: &path::Path, index_file: &mut fs::File) -> anyhow::Result<()> {
    let mut file = fs::File::open(path)?;
    let contents = utils::hash_file_contents(&mut file)?;

    writeln!(index_file, "{}|{}", path.display(), contents).expect("failed to write to index file");

    Ok(())
}

pub fn commit(message: String) -> anyhow::Result<()> {
    let index_path = path::Path::new(".rvc/index");
    let index_file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(index_path).expect("failed to open index file");

    let lines: Vec<_> = BufReader::new(index_file).lines().collect::<Result<_, io::Error>>()?;
    if lines.is_empty() {
        println!("You must add your changes before using commit.");
        return Ok(());
    }

    let current_time = chrono::offset::Utc::now();
    let user_name = whoami::username();
    let user_real_name = whoami::realname();
    let commit_message = format!("{}\n{} - {}\n{}\n", message, user_name, user_real_name, current_time);
    let commit_message_hash = utils::hash_string(commit_message.clone()).expect("failed to hash the commit message");
    let commit_path = format!(".rvc/objects/{}/{}", &commit_message_hash[..2], &commit_message_hash[2..]);
    fs::create_dir_all(&commit_path).expect("failed to create commit directories");

    for line in lines {
        let parts: Vec<&str> = line.split("|").collect(); 
        let path = path::Path::new(parts[0]);

        if path.is_dir() {
            continue;
        }

        // TODO: what should I do with the hash?

        let mut file = fs::File::open(path).expect("failed to open file");
        let compressed_contents = utils::compress_file(&mut file).expect("failed to compress file");

        let file_path = format!("{}/{}", commit_path, path.to_path_buf().display());
        if let Some(parent_dir) = path::Path::new(&file_path).parent() {
            fs::create_dir_all(parent_dir).expect("failed to create parent dir");
        }

        let mut file = fs::File::create(file_path).expect("failed to create file for object");
        file.write_all("blob ".as_bytes()).expect("failed to write 'blob' to file");
        file.write_all(&compressed_contents).expect("failed to write compressed content to file");
    }

    let commit_message_path = format!(".rvc/messages/{}", &commit_message_hash[..2], );
    fs::create_dir(&commit_message_path).expect("failed to create dir for commit message");
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/{}", &commit_message_path, &commit_message_hash[2..])).expect("cannot open file")
        .write_all(&commit_message.as_bytes()).expect("commit message write failed");

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(".rvc/HEAD").expect("failed to open HEAD file")
        .write_all(format!("{}\n", commit_message_hash).as_bytes()).expect("failed to write to HEAD");

    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(".rvc/commits").expect("failed to open 'commits' file")
        .write_all(format!("{}\n", commit_message_hash).as_bytes()).expect("failed to write to 'commits'");

    fs::File::create(index_path).expect("failed to clear index");

    println!("Commit executed successfully.");
    Ok(())
}

pub fn cat_file(_path: String) -> anyhow::Result<()> {
    let path = path::Path::new(&_path);

    if path.is_dir() || !path.is_file() {
        anyhow::bail!("You must enter the path to a valid file.");
    }

    let mut file = fs::File::open(path).expect("failed to open file");
    let blob_prefix = b"blob ";

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("failed to read file to end");

    match buffer.starts_with(blob_prefix) {
        true => {
            buffer = buffer[blob_prefix.len()..].to_vec();
        }
        false => anyhow::bail!("The file path you inserted does not correspond to a blob file."),
    }
    
    let decompressed_data = utils::decompress_content(buffer).expect("failed to decompress file content");
    let decompressed_string = String::from_utf8(decompressed_data).expect("failed to convert decompressed bytes to string");

    println!("{}", decompressed_string);

    Ok(())
}

// ~/rvc_repos
pub fn push() -> anyhow::Result<()> {
    let local_remote = path::Path::new("rvc_repos");
    if local_remote.exists() {
        fs::create_dir_all(local_remote).expect("failed to create repos dir");
    }

    let head_path = path::Path::new(".rvc/HEAD");
    let mut head_file = fs::File::open(head_path).expect("failed to open HEAD file");
    let mut buffer: Vec<u8> = Vec::new();
    head_file.read_to_end(&mut buffer).expect("failed to read the HEAD file");
    let mut current_commit = String::from_utf8(buffer).expect("failed to get string of commit id");
    current_commit.truncate(current_commit.len() - 1);

    let current_commit_objects = format!(".rvc/objects/{}/{}/", &current_commit[..2], &current_commit[2..]);
    let objects_path = path::Path::new(&current_commit_objects);

    if !objects_path.exists() {
        anyhow::bail!("There is no objects directory with the commit id {}", current_commit);
    }

    push_changes(&objects_path, &local_remote).expect("failed to push changes");

    Ok(())
}

fn push_changes(local_dir: &path::Path, remote_dir: &path::Path) -> anyhow::Result<()> {
    if !remote_dir.exists() {
        fs::create_dir_all(remote_dir).expect("failed to create required remote dirs");
    }

    for entry in fs::read_dir(local_dir).expect("failed to read local directory") {
        let entry = entry?;
        let local_path = entry.path();
        let remote_path = remote_dir.join(entry.file_name());

        if local_path.is_dir() {
            push_changes(&local_path, &remote_path)?;
        } else {
            fs::copy(&local_path, &remote_path).expect("failed to copy file from local to remote");
        }
    }

    Ok(())
}
