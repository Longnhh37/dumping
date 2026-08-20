use anyhow::{Ok, Result, anyhow};
use crate::{
    resp::Frame,
    storage::Db,
};

pub struct Del {
    pub keys: Vec<String>,
}

impl Del {
    pub fn parse(args: Vec<Frame>) -> Result<Self> {
        if args.is_empty() {
            return Err(anyhow!("DEL requires at least 1 argument"));
        }

        let mut keys = Vec::new();
        for arg in args {
            match arg {
                Frame::Bulk(Some(bytes)) => keys.push(String::from_utf8(bytes)?),
                _ => return Err(anyhow!("DEL keys must be bulk string")),
            }
        }

        Ok(Del { keys })
    }

    pub fn execute(self, db: &mut Db) -> Frame {
        let count = db.del(&self.keys);
        Frame::Integer(count)
    }
}
