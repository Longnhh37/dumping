use crate::object::{ObjectType, hash_object};
use anyhow::{Context, Result, ensure};

use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    path::Path,
    io::Read,
};

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub ctime_s: u32,
    pub ctime_n: u32,
    pub mtime_s: u32,
    pub mtime_n: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u32,
    pub sha1: [u8; 20],
    pub flags: u16,
    pub path: String,
}

pub fn read_index() -> Result<Vec<IndexEntry>> {
    let path = Path::new(".git/index");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut file = File::open(path)
        .with_context(|| format!("failed to open index file '{}'", path.display()))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .context("failed to read binary index file content")?;
    ensure!(
        data.len() >= 32,
        "index file too short (size: {} bytes, min expected: 32)",
        data.len()
    );
    let (content, checksum) = data.split_at(data.len() - 20);
    let calculated_checksum = Sha1::digest(content);
    ensure!(
        calculated_checksum.as_slice() == checksum,
        "invalid index checksum: index file is corrupted"
    );
    let signature = &content[0..4];
    ensure!(
        signature == b"DIRC",
        "invalid index signature '{:?}', expected 'DIRC'",
        signature
    );
    let version = u32::from_be_bytes(
        content[4..8]
            .try_into()
            .context("failed to parse index version bytes")?,
    );
    ensure!(
        version == 2,
        "unsupported index version {}, expected 2",
        version
    );
    let num_entries = u32::from_be_bytes(
        content[8..12]
            .try_into()
            .context("failed to parse entry count bytes")?,
    ) as usize;
    let entry_data = &content[12..];
    let mut entries = Vec::with_capacity(num_entries);
    let mut i = 0;
    while i + 62 <= entry_data.len() {
        let fields_end = i + 62;
        let ctime_s = u32::from_be_bytes(entry_data[i..i + 4].try_into().unwrap());
        let ctime_n = u32::from_be_bytes(entry_data[i + 4..i + 8].try_into().unwrap());
        let mtime_s = u32::from_be_bytes(entry_data[i + 8..i + 12].try_into().unwrap());
        let mtime_n = u32::from_be_bytes(entry_data[i + 12..i + 16].try_into().unwrap());
        let dev = u32::from_be_bytes(entry_data[i + 16..i + 20].try_into().unwrap());
        let ino = u32::from_be_bytes(entry_data[i + 20..i + 24].try_into().unwrap());
        let mode = u32::from_be_bytes(entry_data[i + 24..i + 28].try_into().unwrap());
        let uid = u32::from_be_bytes(entry_data[i + 28..i + 32].try_into().unwrap());
        let gid = u32::from_be_bytes(entry_data[i + 32..i + 36].try_into().unwrap());
        let size = u32::from_be_bytes(entry_data[i + 36..i + 40].try_into().unwrap());
        let mut sha1 = [0u8; 20];
        sha1.copy_from_slice(&entry_data[i + 40..i + 60]);
        let flags = u16::from_be_bytes(entry_data[i + 60..i + 62].try_into().unwrap());
        let null_pos = entry_data[fields_end..]
            .iter()
            .position(|&b| b == 0)
            .context("missing null terminator in index entry path string")?;
        let path_bytes = &entry_data[fields_end..fields_end + null_pos];
        let path_str = String::from_utf8(path_bytes.to_vec())
            .context("invalid UTF-8 sequence in index path string")?;
        entries.push(IndexEntry {
            ctime_s,
            ctime_n,
            mtime_s,
            mtime_n,
            dev,
            ino,
            mode,
            uid,
            gid,
            size,
            sha1,
            flags,
            path: path_str,
        });
        let entry_len = ((62 + path_bytes.len() + 8) / 8) * 8;
        i += entry_len;
    }
    Ok(entries)
}

pub fn write_index(entries: &[IndexEntry]) -> Result<()> {
    let mut packed: Vec<u8> = Vec::new();
    packed.extend_from_slice(b"DIRC");
    packed.extend_from_slice(&2u32.to_be_bytes());
    packed.extend_from_slice(&(entries.len() as u32).to_be_bytes());
    for entry in entries {
        packed.extend_from_slice(&entry.ctime_s.to_be_bytes());
        packed.extend_from_slice(&entry.ctime_n.to_be_bytes());
        packed.extend_from_slice(&entry.mtime_s.to_be_bytes());
        packed.extend_from_slice(&entry.mtime_n.to_be_bytes());
        packed.extend_from_slice(&entry.dev.to_be_bytes());
        packed.extend_from_slice(&entry.ino.to_be_bytes());
        packed.extend_from_slice(&entry.mode.to_be_bytes());
        packed.extend_from_slice(&entry.uid.to_be_bytes());
        packed.extend_from_slice(&entry.gid.to_be_bytes());
        packed.extend_from_slice(&entry.size.to_be_bytes());
        packed.extend_from_slice(&entry.sha1);
        packed.extend_from_slice(&entry.flags.to_be_bytes());
        let path_bytes = entry.path.as_bytes();
        packed.extend_from_slice(path_bytes);
        let length = ((62 + path_bytes.len() + 8) / 8) * 8;
        let pad_len = length - 62 - path_bytes.len();
        packed.resize(packed.len() + pad_len, 0);
    }
    let checksum = Sha1::digest(&packed);
    packed.extend_from_slice(&checksum);
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        fs::create_dir_all(git_dir).context("failed to create '.git' directory")?;
    }
    fs::write(git_dir.join("index"), packed).context("failed to write data to '.git/index'")?;
    Ok(())
}

/// Add given file paths to index (replacing existing entries for same path), re-sorted by path.
pub fn add(paths: &[String]) -> Result<()> {
    let normalized_paths: Vec<String> = paths.iter().map(|p| p.replace('\\', "/")).collect();
    let all_entries =
        read_index().context("failed to read current index state for add operation")?;
    let mut entries: Vec<IndexEntry> = all_entries
        .into_iter()
        .filter(|e| !normalized_paths.contains(&e.path))
        .collect();
    for path in &normalized_paths {
        let content =
            fs::read(path).with_context(|| format!("failed to read target file '{}'", path))?;
        let sha1_hex = hash_object(&content, ObjectType::Blob, true)
            .with_context(|| format!("failed to store object for file '{}'", path))?;
        let sha1_bytes_vec = hex::decode(&sha1_hex)
            .with_context(|| format!("invalid hex hash string '{}'", sha1_hex))?;
        let sha1_bytes: [u8; 20] = sha1_bytes_vec
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid SHA-1 length, expected 20 bytes"))?;
        let metadata = fs::metadata(path)
            .with_context(|| format!("failed to query metadata for file '{}'", path))?;
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;
        #[cfg(unix)]
        let (ctime_s, ctime_n, mtime_s, mtime_n, dev, ino, mode, uid, gid) = (
            metadata.ctime() as u32,
            metadata.ctime_nsec() as u32,
            metadata.mtime() as u32,
            metadata.mtime_nsec() as u32,
            metadata.dev() as u32,
            metadata.ino() as u32,
            metadata.mode(),
            metadata.uid(),
            metadata.gid(),
        );
        #[cfg(not(unix))]
        let (ctime_s, ctime_n, mtime_s, mtime_n, dev, ino, mode, uid, gid) =
            (0, 0, 0, 0, 0, 0, 0o100644, 0, 0);
        let flags = (path.len() as u16) & 0x0FFF;
        entries.push(IndexEntry {
            ctime_s,
            ctime_n,
            mtime_s,
            mtime_n,
            dev,
            ino,
            mode,
            uid,
            gid,
            size: metadata.len() as u32,
            sha1: sha1_bytes,
            flags,
            path: path.clone(),
        });
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    write_index(&entries).context("failed to write updated entries to index")
}

/// Print index entries (path only, or with details).
pub fn ls_files(details: bool) -> Result<()> {
    let entries = read_index().context("failed to read index for ls-files command")?;
    for entry in entries {
        if details {
            let stage = (entry.flags >> 12) & 3;
            println!(
                "{:06o} {} {}\t{}",
                entry.mode,
                hex::encode(entry.sha1),
                stage,
                entry.path
            );
        } else {
            println!("{}", entry.path);
        }
    }
    Ok(())
}
