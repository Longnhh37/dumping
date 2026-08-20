pub mod del;
pub mod dispatcher;
pub mod get;
pub mod set;

use crate::resp::Frame;
use crate::storage::Db;
use anyhow::Result;

pub enum Command {
    Get(get::Get),
    Set(set::Set),
    Del(del::Del),
}

impl Command {
    pub fn from_frame(frame: Frame) -> Result<Self> {
        dispatcher::dispatch(frame)
    }

    pub fn execute(self, db: &mut Db) -> Frame {
        match self {
            Command::Get(cmd) => cmd.execute(db),
            Command::Set(cmd) => cmd.execute(db),
            Command::Del(cmd) => cmd.execute(db),
        }
    }
}
