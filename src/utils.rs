use flate2::{self, Compression};
use sha2::Digest;
use std::{fs, io::{Read, Write}};

pub fn hash_file_contents(file: &mut fs::File) -> anyhow::Result<String> {
    let mut hasher = sha2::Sha256::new();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    hasher.update(&buffer);
    let hash_result = hasher.finalize();

    Ok(format!("{:x}", hash_result))
}

pub fn hash_string(str: String) -> anyhow::Result<String> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(str);
    
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn compress_file(file: &mut fs::File) -> anyhow::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("failed to read file");

    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer).expect("failed to write to encoder");
    let compressed_contents = encoder.finish().expect("failed to compress file contents");

    Ok(compressed_contents)
}

pub fn decompress_content(compressed_bytes: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let data_cursor = std::io::Cursor::new(compressed_bytes);
    let mut decoder =  flate2::read::ZlibDecoder::new(data_cursor);
    let mut decompressed_data = Vec::new();

    decoder.read_to_end(&mut decompressed_data).expect("failed to read to end and decode data");

    Ok(decompressed_data)
}
