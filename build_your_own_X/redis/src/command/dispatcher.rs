use crate::command::Command;
use crate::resp::Frame;
use anyhow::{Result, anyhow};

pub fn dispatch(frame: Frame) -> Result<Command> {
    let arr = match frame {
        Frame::Array(Some(frames)) => frames,
        _ => return Err(anyhow!("command must be an array")),
    };

    if arr.is_empty() {
        return Err(anyhow!("empty command"));
    }

    let mut iter = arr.into_iter();

    let name = match iter.next().unwrap() {
        Frame::Bulk(Some(bytes)) => String::from_utf8(bytes)?.to_uppercase(),
        _ => return Err(anyhow!("command name must be a bulk string")),
    };


    let args: Vec<Frame> = iter.collect();

    match name.as_str() {
        "GET" => Ok(Command::Get(super::get::Get::parse(args)?)),
        "SET" => Ok(Command::Set(super::set::Set::parse(args)?)),
        "DEL" => Ok(Command::Del(super::del::Del::parse(args)?)),
        _ => Err(anyhow!("unknown command: {}", name)),
    }
}

