use crate::{
    resp::Frame,
    storage::Db,
};
use anyhow::{anyhow, Result};

pub struct Get {
    pub key: String,
}

impl Get {
    pub fn parse(args: Vec<Frame>) -> Result<Self> {
        if args.len() != 1 {
            return Err(anyhow!("GET requires exactly 1 argument"));
        }

        let key = match args.into_iter().next().unwrap() {
            Frame::Bulk(Some(bytes)) =>  String::from_utf8(bytes)?,
            _ => return Err(anyhow!("GET key must be a bulk string")),
        };

        Ok(Get { key })
    }

    pub fn execute(self, db: &mut Db) -> Frame {
        match db.get(&self.key) {
            Some(value) => Frame::Bulk(Some(value)),
            None => Frame::Bulk(None),
        }
    }
}
