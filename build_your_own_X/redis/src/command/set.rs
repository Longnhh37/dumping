use crate::{resp::Frame, storage::Db};
use anyhow::{Result, anyhow};

pub struct Set {
    pub key: String,
    pub value: Vec<u8>,
}

impl Set {
    pub fn parse(args: Vec<Frame>) -> Result<Self> {
        if args.len() != 2 {
            return Err(anyhow!("SET requires exactly 2 arguments"));
        }

        let mut iter = args.into_iter();

        let key = match iter.next().unwrap() {
            Frame::Bulk(Some(bytes)) => String::from_utf8(bytes)?,
            _ => return Err(anyhow!("SET key must be a bulk string")),
        };

        let value = match iter.next().unwrap() {
            Frame::Bulk(Some(bytes)) => bytes,
            _ => return Err(anyhow!("SET value must be a bulk string")),
        };

        Ok(Set { key, value })
    }

    pub fn execute(self, db: &mut Db) -> Frame {
        db.set(self.key, self.value);
        Frame::Simple("OK".to_string())
    }
}
