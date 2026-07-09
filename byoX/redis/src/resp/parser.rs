use crate::resp::Frame;
use anyhow::anyhow;

pub enum ParseResult {
    Complete(Frame, usize),
    Incomplete,
    Error(anyhow::Error),
}

pub fn parse(buf: &[u8]) -> ParseResult {
    if buf.is_empty() {
        return ParseResult::Incomplete;
    }

    // idx = index postition of '\r' for the first '\r\n'
    let idx = match find_crlf(buf) {
        Some(pos) => pos,
        None => return ParseResult::Incomplete,
    };

    let line = match std::str::from_utf8(&buf[1..idx]) {
        Ok(s) => s,
        Err(e) => return ParseResult::Error(e.into()),
    };

    match buf[0] {
        // +OK\r\n -> consumed = 1 + content + 2
        b'+' => ParseResult::Complete(Frame::Simple(line.to_string()), idx + 2),

        b'-' => ParseResult::Complete(Frame::Error(line.to_string()), idx + 2),

        b':' => match line.parse::<i64>() {
            Ok(n) => ParseResult::Complete(Frame::Integer(n), idx + 2),
            Err(e) => ParseResult::Error(e.into()),
        },

        b'$' => match line.parse::<i64>() {
            Ok(-1) => ParseResult::Complete(Frame::Bulk(None), idx + 2),
            Ok(n) if n >= 0 => {
                let n = n as usize;
                let data_start = idx + 2;
                let data_end = data_start + n;
                let frame_end = data_end + 2;

                if buf.len() < frame_end {
                    return ParseResult::Incomplete;
                }
                let data = buf[data_start..data_end].to_vec();
                ParseResult::Complete(Frame::Bulk(Some(data)), frame_end)
            }
            Ok(_) => ParseResult::Error(anyhow!("invalid bulk length")),
            Err(e) => ParseResult::Error(e.into()),
        },

        b'*' => match line.parse::<i64>() {
            Ok(-1) => ParseResult::Complete(Frame::Array(None), idx + 2),
            Ok(n) if n >= 0 => {
                let n = n as usize;
                let mut frames = Vec::with_capacity(n);
                let mut cursor = idx + 2;

                for _ in 0..n {
                    match parse(&buf[cursor..]) {
                        ParseResult::Complete(frame, consumed) => {
                            frames.push(frame);
                            cursor += consumed;
                        }
                        other => return other,
                    }
                }

                ParseResult::Complete(Frame::Array(Some(frames)), cursor)
            }
            Ok(_) => ParseResult::Error(anyhow!("invalid array length")),
            Err(e) => ParseResult::Error(e.into()),
        },

        _ => ParseResult::Error(anyhow!("invalid first byte: {}", buf[0])),
    }
}

fn find_crlf(buf: &[u8]) -> Option<usize> {
    buf.windows(2).position(|w| w == b"\r\n")
}
