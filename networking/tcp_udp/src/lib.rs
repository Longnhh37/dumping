use std::io::{self, ErrorKind};

/// In `msg` ra stdout rồi đọc một dòng từ stdin (bỏ ký tự newline ở cuối).
pub fn prompt(msg: &str) -> io::Result<String> {
    println!("{msg}");
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        return Err(io::Error::new(ErrorKind::UnexpectedEof, "stdin closed"));
    }
    Ok(line.trim_end_matches(['\r', '\n']).to_owned())
}

/// Hỏi host + port, trả về `(host, port)`.
pub fn prompt_address(host_msg: &str) -> Result<(String, u16), Box<dyn std::error::Error>> {
    let host = prompt(host_msg)?;
    let port = prompt("Enter a server port (e.g. 12345):")?.trim().parse::<u16>()?;
    Ok((host, port))
}
