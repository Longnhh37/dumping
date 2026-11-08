// Compile-time lookup table: Base64 decode table
const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const INVALID: u8 = 0xFF;

const fn decode_table(alphabet: &[u8; 64]) -> [u8; 256] {
    let mut table = [INVALID; 256];
    let mut i = 0;
    while i < 64 {
        let c = alphabet[i] as usize;
        assert!(table[c] == INVALID, "duplicate character in alphabet");
        table[c] = i as u8;
        i += 1;
    }
    table
}

const DECODE: [u8; 256] = decode_table(ALPHABET);

// proofs about the generated table, checked during compilation.
const _: () = {
    // 1. The table is the exact inverse of the alphabet.
    let mut i = 0;
    while i < 64 {
        assert!(
            DECODE[ALPHABET[i] as usize] as usize == i,
            "table is not the inverse of the alphabet"
        );
        i += 1;
    }

    // 2. Exactly 64 byte values are valid. Everything else is INVALID.
    let (mut valid, mut b) = (0, 0);
    while b < 256 {
        if DECODE[b] != INVALID {
            valid += 1;
        }
        b += 1;
    }
    assert!(valid == 64, "table must have exactly 64 valid entries");

    // 3. The padding character is not a data symbol.
    assert!(DECODE[b'=' as usize] == INVALID, "'=' must not be a symbol");
};

fn decode_quad(quad: [u8; 4]) -> Option<[u8; 3]> {
    let mut acc: u32 = 0;
    for c in quad {
        let v = DECODE[c as usize];
        if v == INVALID {
            return None;
        }
        acc = (acc << 6) | u32::from(v);
    }
    Some([(acc >> 16) as u8, (acc >> 8) as u8, acc as u8])
}

fn main() {
    let out = decode_quad(*b"TWFu").unwrap();
    println!("base64 'TWFu' -> {}", String::from_utf8_lossy(&out));
    println!("base64 'TW=u' -> {:?}", decode_quad(*b"TW=u"));
}
