use anyhow::{Context, Result, bail, ensure};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Commit,
    Tree,
    Blob,
}

impl ObjectType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "commit" => Ok(Self::Commit),
            "tree" => Ok(Self::Tree),
            "blob" => Ok(Self::Blob),
            _ => bail!("invalid object type '{}'", s),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::Tree => "tree",
            Self::Blob => "blob",
        }
    }

    pub fn pack_type_num(&self) -> u8 {
        match self {
            Self::Commit => 1,
            Self::Tree => 2,
            Self::Blob => 3,
        }
    }
}

pub fn hash_object(data: &[u8], obj_type: ObjectType, write: bool) -> Result<String> {
    let header = format!("{} {}", obj_type.as_str(), data.len());

    let mut full_data = Vec::with_capacity(header.len() + 1 + data.len());
    full_data.extend_from_slice(header.as_bytes());
    full_data.push(b'\0');
    full_data.extend_from_slice(data);

    let hash = Sha1::digest(&full_data);
    let sha1_hex = hex::encode(hash);

    if write {
        let path = PathBuf::from(".git")
            .join("objects")
            .join(&sha1_hex[..2])
            .join(&sha1_hex[2..]);

        if !path.exists() {
            let dir = path.parent().context("object path has no parent")?;
            fs::create_dir_all(dir)
                .with_context(|| format!("failed to create dir '{}'", dir.display()))?;

            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&full_data)?;
            let compressed = encoder.finish()?;

            fs::write(&path, compressed)
                .with_context(|| format!("failed to write object '{}'", path.display()))?;
        }
    }
    Ok(sha1_hex)
}

pub fn find_object(sha1_prefix: &str) -> Result<PathBuf> {
    ensure!(
        sha1_prefix.len() >= 2,
        "hash prefix must be 2 or more characters"
    );

    let obj_dir = PathBuf::from(".git")
        .join("objects")
        .join(&sha1_prefix[..2]);
    let rest = &sha1_prefix[2..];

    let matches: Vec<PathBuf> = fs::read_dir(&obj_dir)
        .with_context(|| format!("failed to read '{}'", obj_dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name.starts_with(rest))
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect();

    match matches.len() {
        0 => bail!("object '{}' not found", sha1_prefix),
        1 => Ok(matches[0].clone()),
        n => bail!(
            "multiple object ({}) with prefix '{}' found",
            n,
            sha1_prefix
        ),
    }
}

pub fn read_object(sha1_prefix: &str) -> Result<(ObjectType, Vec<u8>)> {
    let path = find_object(sha1_prefix)?;
    let compressed = fs::read(&path)
        .with_context(|| format!("failed to read object file '{}'", path.display()))?;

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut full_data = Vec::new();
    decoder
        .read_to_end(&mut full_data)
        .context("failed to zlib-decompress object")?;

    let nul_index = full_data
        .iter()
        .position(|&b| b == 0)
        .context("object missing null-bye header delimiter")?;

    let header =
        std::str::from_utf8(&full_data[..nul_index]).context("object header is not valid utf-8")?;

    let mut parts = header.split(' ');
    let type_str = parts.next().context("object header missing type")?;
    let size_str = parts.next().context("object header missing size")?;

    let obj_type = ObjectType::from_str(type_str)?;
    let size: usize = size_str
        .parse()
        .with_context(|| format!("invalid size '{}' in object header", size_str))?;

    let data = full_data[nul_index + 1..].to_vec();

    ensure!(
        size == data.len(),
        "expected size {}, got {} bytes",
        size,
        data.len()
    );

    Ok((obj_type, data))
}

pub fn cat_file(mode: &str, sha1_prefix: &str) -> Result<()> {
    let (obj_type, data) = read_object(sha1_prefix)?;

    match mode {
        "commit" | "tree" | "blob" => {
            ensure!(
                obj_type.as_str() == mode,
                "expected object type {}, got {}",
                mode,
                obj_type.as_str(),
            );
            std::io::stdout().write_all(&data)?;
        }
        "size" => println!("{}", data.len()),
        "type" => println!("{}", obj_type.as_str()),
        "pretty" => match obj_type {
            ObjectType::Commit | ObjectType::Blob => std::io::stdout().write_all(&data)?,
            ObjectType::Tree => {
                for (entry_mode, path, sha1) in read_tree(None, Some(&data))? {
                    let is_dir = (entry_mode & 0o170000) == 0o040000;
                    let type_str = if is_dir { "tree" } else { "blob" };
                    println!("{:06o} {} {}\t{}", entry_mode, type_str, sha1, path);
                }
            }
        },
        other => bail!("unexpected mode '{}'", other),
    }

    Ok(())
}

pub type TreeEntry = (u32, String, String);

pub fn read_tree(sha1: Option<&str>, data: Option<&[u8]>) -> Result<Vec<TreeEntry>> {
    let raw_data = match (sha1, data) {
        (Some(prefix), None) => {
            let (obj_type, bytes) = read_object(prefix)?;
            ensure!(
                obj_type == ObjectType::Tree,
                "object '{}' is not a tree object",
                prefix
            );
            bytes
        }
        (None, Some(bytes)) => bytes.to_vec(),
        _ => bail!("must provide either sha1 or raw data to read_tree"),
    };

    let mut entries = Vec::new();
    let mut cursor = 0;

    while cursor < raw_data.len() {
        let space_pos = raw_data[cursor..]
            .iter()
            .position(|&b| b == b' ')
            .map(|p| cursor + p)
            .context("invalid tree entry: missing space after mode")?;

        let mode_str = std::str::from_utf8(&raw_data[cursor..space_pos])
            .context("invalid mode UTF-8 in tree entry")?;
        let mode = u32::from_str_radix(mode_str, 8)
            .with_context(|| format!("invalid octal mode '{}'", mode_str))?;

        let null_pos = raw_data[space_pos + 1..]
            .iter()
            .position(|&b| b == b'\0')
            .map(|p| space_pos + 1 + p)
            .context("invalid tree entry: missing null byte after path")?;

        let path = std::str::from_utf8(&raw_data[space_pos + 1..null_pos])
            .context("invalid path UTF-8 in tree entry")?
            .to_string();

        let sha_start = null_pos + 1;
        let sha_end = sha_start + 20;

        ensure!(
            sha_end <= raw_data.len(),
            "truncated SHA-1 binary bytes in tree entry"
        );

        let sha1_hex = hex::encode(&raw_data[sha_start..sha_end]);
        entries.push((mode, path, sha1_hex));

        cursor = sha_end;
    }

    Ok(entries)
}

pub fn write_tree() -> Result<String> {
    let index_entries =
        crate::index::read_index().context("failed to read index for writing tree")?;
    let mut tree_buf = Vec::new();

    for entry in index_entries {
        ensure!(
            !entry.path.contains('/'),
            "currently write_tree only supports top-level files (path contains '/': {})",
            entry.path
        );

        let mode_path = format!("{:o} {}", entry.mode, entry.path);
        tree_buf.extend_from_slice(mode_path.as_bytes());
        tree_buf.push(b'\0');
        tree_buf.extend_from_slice(&entry.sha1);
    }

    hash_object(&tree_buf, ObjectType::Tree, true).context("failed to hash and store tree object")
}

pub fn find_tree_objects(tree_sha1: &str) -> Result<HashSet<String>> {
    let mut result = HashSet::new();
    result.insert(tree_sha1.to_string());

    let entries = read_tree(Some(tree_sha1), None)
        .with_context(|| format!("failed to read tree object '{}'", tree_sha1))?;

    for (mode, _, entry_sha1) in entries {
        if (mode & 0o170000) == 0o040000 {
            let sub_objects = find_tree_objects(&entry_sha1)?;
            result.extend(sub_objects);
        } else {
            result.insert(entry_sha1);
        }
    }

    Ok(result)
}

pub fn find_commit_objects(commit_sha1: &str) -> Result<HashSet<String>> {
    let mut result = HashSet::new();
    let mut queue = vec![commit_sha1.to_string()];

    while let Some(cur_sha) = queue.pop() {
        if !result.insert(cur_sha.clone()) {
            continue;
        }

        let (obj_type, data) = read_object(&cur_sha)
            .with_context(|| format!("failed to read commit object '{}'", cur_sha))?;

        if obj_type != ObjectType::Commit {
            continue;
        }

        let content =
            std::str::from_utf8(&data).context("invalid UTF-8 sequence in commit content")?;

        for line in content.lines() {
            if line.is_empty() {
                break;
            }
            if let Some(tree_sha) = line.strip_prefix("tree ") {
                let tree_objs = find_tree_objects(tree_sha.trim())?;
                result.extend(tree_objs);
            } else if let Some(parent_sha) = line.strip_prefix("parent ") {
                queue.push(parent_sha.trim().to_string());
            }
        }
    }

    Ok(result)
}

pub fn find_missing_objects(
    local_sha1: &str,
    remote_sha1: Option<&str>,
) -> Result<HashSet<String>> {
    let local = find_commit_objects(local_sha1)
        .with_context(|| format!("failed to map local commit objects for '{}'", local_sha1))?;

    if let Some(remote_sha) = remote_sha1 {
        let remote = find_commit_objects(remote_sha)
            .with_context(|| format!("failed to map remote commit objects for '{}'", remote_sha))?;
        Ok(local.difference(&remote).cloned().collect())
    } else {
        Ok(local)
    }
}

pub fn encode_pack_object(sha1: &str) -> Result<Vec<u8>> {
    let (obj_type, data) = read_object(sha1)?;
    let type_num = obj_type.pack_type_num();
    let mut size = data.len();

    let mut header = Vec::new();
    let mut byte = ((type_num & 0b111) << 4) | ((size & 0b1111) as u8);
    size >>= 4;

    if size > 0 {
        byte |= 0x80;
    }
    header.push(byte);

    while size > 0 {
        let mut b = (size & 0x7F) as u8;
        size >>= 7;
        if size > 0 {
            b |= 0x80;
        }
        header.push(b);
    }

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&data)
        .context("failed to write data to pack zlib encoder")?;
    let compressed = encoder
        .finish()
        .context("failed to compress pack object data with zlib")?;

    header.extend(compressed);
    Ok(header)
}

pub fn create_pack(objects: &std::collections::HashSet<String>) -> Result<Vec<u8>> {
    let mut pack_data = Vec::new();

    pack_data.extend_from_slice(b"PACK");
    pack_data.extend_from_slice(&2u32.to_be_bytes());
    pack_data.extend_from_slice(&(objects.len() as u32).to_be_bytes());

    let mut sorted_objs: Vec<_> = objects.iter().collect();
    sorted_objs.sort();

    for sha1 in sorted_objs {
        let entry = encode_pack_object(sha1)
            .with_context(|| format!("failed to encode project '{}' for packfile", sha1))?;
        pack_data.extend(entry);
    }

    let checksum = Sha1::digest(&pack_data);
    pack_data.extend_from_slice(&checksum);

    Ok(pack_data)
}
