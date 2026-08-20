// storage/file.rs
use anyhow::Result;

use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

use crate::storage::page::PAGE_SIZE;

pub struct DbFile {
    file: File,
}

impl DbFile {
    pub fn open(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)?;

        Ok(Self { file })
    }

    pub fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buf)?;

        Ok(())
    }

    pub fn write_at(&mut self, offset: u64, buf: &[u8]) -> Result<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(buf)?;

        Ok(())
    }

    pub fn sync(&mut self) -> Result<()> {
        self.file.sync_all()?;
        Ok(())
    }

    pub fn len(&mut self) -> Result<u64> {
        Ok(self.file.seek(SeekFrom::End(0))?)
    }

    pub fn num_pages(&mut self) -> Result<u64> {
        Ok(self.len()? / PAGE_SIZE as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    // ----- helper -----
    fn temp_db() -> (DbFile, NamedTempFile) {
        let tmp = NamedTempFile::new().expect("cannot create temp file");
        let db = DbFile::open(tmp.path().to_str().unwrap()).expect("cannot open DbFile");

        (db, tmp)
    }

    // ----- open() -----
    #[test]
    fn test_open_creates_file_if_not_exists() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new.db");

        let result = DbFile::open(path.to_str().unwrap());

        assert!(result.is_ok());
        assert!(path.exists(), "file has to be created on disk");
    }

    #[test]
    fn test_open_existing_file_does_not_truncate() {
        let (mut db, tmp) = temp_db();
        db.write_at(0, b"hello").unwrap();
        drop(db);

        let mut db2 = DbFile::open(tmp.path().to_str().unwrap()).unwrap();
        let mut buf = [0u8; 5];
        db2.read_at(0, &mut buf).unwrap();

        assert_eq!(&buf, b"hello");
    }

    // ----- write_at() & read_at() -----
    #[test]
    fn test_write_then_read_roundtrip() {
        let (mut db, _tmp) = temp_db();
        let data = b"rust database";

        db.write_at(0, data).unwrap();

        let mut buf = vec![0u8; data.len()];
        db.read_at(0, &mut buf).unwrap();
        assert_eq!(buf, data);
    }

    #[test]
    fn test_write_at_arbitrary_offset() {
        let (mut db, _tmp) = temp_db();

        db.write_at(0, &[0u8; 100]).unwrap(); // pad 100 bytes trước
        db.write_at(64, b"page2").unwrap();

        let mut buf = [0u8; 5];
        db.read_at(64, &mut buf).unwrap();
        assert_eq!(&buf, b"page2");
    }

    #[test]
    fn test_write_multiple_regions_independent() {
        let (mut db, _tmp) = temp_db();

        db.write_at(0, b"AAAA").unwrap();
        db.write_at(4, b"BBBB").unwrap();
        db.write_at(8, b"CCCC").unwrap();

        let mut a = [0u8; 4];
        let mut b = [0u8; 4];
        let mut c = [0u8; 4];
        db.read_at(0, &mut a).unwrap();
        db.read_at(4, &mut b).unwrap();
        db.read_at(8, &mut c).unwrap();

        assert_eq!(&a, b"AAAA");
        assert_eq!(&b, b"BBBB");
        assert_eq!(&c, b"CCCC");
    }

    #[test]
    fn test_overwrite_existing_data() {
        let (mut db, _tmp) = temp_db();

        db.write_at(0, b"AABBCC").unwrap();
        db.write_at(2, b"XX").unwrap();

        let mut buf = [0u8; 6];
        db.read_at(0, &mut buf).unwrap();
        assert_eq!(&buf, b"AAXXCC");
    }

    // ── read_at() error cases ──────────────────────────────
    #[test]
    fn test_read_beyond_eof_returns_error() {
        let (mut db, _tmp) = temp_db();
        db.write_at(0, b"hi").unwrap();

        let mut buf = [0u8; 10];
        let result = db.read_at(0, &mut buf);

        assert!(result.is_err(), "đọc quá EOF phải trả lỗi");
    }

    #[test]
    fn test_read_empty_file_returns_error() {
        let (mut db, _tmp) = temp_db();

        let mut buf = [0u8; 1];
        let result = db.read_at(0, &mut buf);

        assert!(result.is_err());
    }

    // ── len() ─────────────────────────────────────────────────
    #[test]
    fn test_len_empty_file_is_zero() {
        let (mut db, _tmp) = temp_db();
        assert_eq!(db.len().unwrap(), 0);
    }

    #[test]
    fn test_len_reflects_written_data() {
        let (mut db, _tmp) = temp_db();
        db.write_at(0, b"12345").unwrap();
        assert_eq!(db.len().unwrap(), 5);
    }

    #[test]
    fn test_len_after_sparse_write() {
        let (mut db, _tmp) = temp_db();
        db.write_at(100, b"hello").unwrap();
        assert!(db.len().unwrap() >= 105);
    }

    // ── sync() ───────────────────────────────────────────────
    #[test]
    fn test_sync_does_not_corrupt_data() {
        let (mut db, _tmp) = temp_db();
        db.write_at(0, b"persist").unwrap();
        db.sync().unwrap();

        let mut buf = [0u8; 7];
        db.read_at(0, &mut buf).unwrap();
        assert_eq!(&buf, b"persist");
    }

    #[test]
    fn test_data_persists_after_reopen() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap().to_string();

        {
            let mut db = DbFile::open(&path).unwrap();
            db.write_at(0, b"durable").unwrap();
            db.sync().unwrap();
        }

        let mut db2 = DbFile::open(&path).unwrap();
        let mut buf = [0u8; 7];
        db2.read_at(0, &mut buf).unwrap();
        assert_eq!(&buf, b"durable");
    }
}
