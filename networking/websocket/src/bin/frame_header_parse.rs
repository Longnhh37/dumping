#[derive(Debug)]
struct FrameHeader {
    fin: bool,
    opcode: u8,
    masking_key: Option<[u8; 4]>,
    payload_len: u64,
    header_len: usize,
}

fn parse_header(buf: &[u8]) -> Option<FrameHeader> {
    let b0 = *buf.first()?;
    let b1 = *buf.get(1)?;

    let fin = b0 & 0x80 != 0;
    let opcode = b0 & 0x0F;
    let masked = b1 & 0x80 != 0;

    let mut offset = 2;
    let payload_len = match b1 & 0x7F {
        126 => {
            let v = u16::from_be_bytes(buf.get(2..4)?.try_into().ok()?);
            offset = 4;
            u64::from(v)
        }
        127 => {
            let v = u64::from_be_bytes(buf.get(2..10)?.try_into().ok()?);
            offset = 10;
            v
        }
        n => u64::from(n),
    };

    let masking_key = if masked {
        let key: [u8; 4] = buf.get(offset..offset + 4)?.try_into().ok()?;
        offset += 4;
        Some(key)
    } else {
        None
    };

    Some(FrameHeader {
        fin,
        opcode,
        masking_key,
        payload_len,
        header_len: offset,
    })
}

fn unmask(payload: &mut [u8], key: [u8; 4]) {
    for (i, byte) in payload.iter_mut().enumerate() {
        *byte ^= key[i % 4];
    }
}

fn main() {
    let mut frame = vec![
        0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58,
    ];
    let h = parse_header(&frame).unwrap();
    println!("{h:?}");

    let key = h.masking_key.unwrap();
    let payload = &mut frame[h.header_len..];
    unmask(payload, key);
    println!("{}", String::from_utf8_lossy(payload));
}
