use crate::resp::Frame;

pub fn encode(frame: &Frame) -> Vec<u8> {
    let mut buf = Vec::new();
    encode_into(frame, &mut buf);
    buf
}

fn encode_into(frame: &Frame, buf: &mut Vec<u8>) {
    match frame {
        Frame::Simple(s) => {
            buf.push(b'+');
            buf.extend_from_slice(s.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        Frame::Error(s) => {
            buf.push(b'-');
            buf.extend_from_slice(s.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        Frame::Integer(n) => {
            buf.push(b':');
            buf.extend_from_slice(n.to_string().as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        Frame::Bulk(None) => buf.extend_from_slice(b"$-1\r\n"),

        Frame::Bulk(Some(data)) => {
            buf.push(b'$');
            buf.extend_from_slice(data.len().to_string().as_bytes());
            buf.extend_from_slice(b"\r\n");
            buf.extend_from_slice(data);
            buf.extend_from_slice(b"\r\n");
        }

        Frame::Array(None) => buf.extend_from_slice(b"*-1\r\n"),

        Frame::Array(Some(frames)) => {
            buf.push(b'*');
            buf.extend_from_slice(frames.len().to_string().as_bytes());
            buf.extend_from_slice(b"\r\n");
            for frame in frames {
                encode_into(frame, buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::frame::Frame;

    #[test]
    fn test_simple_string() {
        assert_eq!(encode(&Frame::Simple("OK".into())), b"+OK\r\n");
    }

    #[test]
    fn test_error() {
        assert_eq!(encode(&Frame::Error("ERR bad".into())), b"-ERR bad\r\n");
    }

    #[test]
    fn test_integer() {
        assert_eq!(encode(&Frame::Integer(42)), b":42\r\n");
        assert_eq!(encode(&Frame::Integer(-1)), b":-1\r\n");
    }

    #[test]
    fn test_bulk_null() {
        assert_eq!(encode(&Frame::Bulk(None)), b"$-1\r\n");
    }

    #[test]
    fn test_bulk_string() {
        assert_eq!(
            encode(&Frame::Bulk(Some(b"foo".to_vec()))),
            b"$3\r\nfoo\r\n"
        );
    }

    #[test]
    fn test_bulk_binary() {
        // binary data chứa \r\n bên trong vẫn encode đúng
        let data = b"he\r\no".to_vec();
        assert_eq!(
            encode(&Frame::Bulk(Some(data))),
            b"$5\r\nhe\r\no\r\n"
        );
    }

    #[test]
    fn test_array_null() {
        assert_eq!(encode(&Frame::Array(None)), b"*-1\r\n");
    }

    #[test]
    fn test_array() {
        // SET foo bar → *3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n
        let frame = Frame::Array(Some(vec![
            Frame::Bulk(Some(b"SET".to_vec())),
            Frame::Bulk(Some(b"foo".to_vec())),
            Frame::Bulk(Some(b"bar".to_vec())),
        ]));
        assert_eq!(
            encode(&frame),
            b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"
        );
    }

    #[test]
    fn test_nested_array() {
        let frame = Frame::Array(Some(vec![
            Frame::Integer(1),
            Frame::Array(Some(vec![
                Frame::Simple("OK".into()),
            ])),
        ]));
        assert_eq!(
            encode(&frame),
            b"*2\r\n:1\r\n*1\r\n+OK\r\n"
        );
    }
}
