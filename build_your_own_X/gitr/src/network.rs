use anyhow::{Result, Context, ensure};
use std::io::Read;

/// Parse pkt-line format from server response into list of raw lines.
pub fn extract_lines(data: &[u8]) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    let mut i = 0;
    for _ in 0..1000 {
        if i + 4 > data.len() {
            break;
        }
        let length_str = match std::str::from_utf8(&data[i..i + 4]) {
            Ok(s) => s,
            Err(_) => break,
        };
        let line_len = match usize::from_str_radix(length_str, 16) {
            Ok(l) => l,
            Err(_) => break,
        };
        if line_len == 0 {
            lines.push(Vec::new());
            i += 4;
        } else {
            if i + line_len > data.len() {
                break;
            }
            let line = data[i + 4..i + line_len].to_vec();
            lines.push(line);
            i += line_len;
        }
        if i >= data.len() {
            break;
        }
    }
    lines
}

/// Encode lines into pkt-line format to send to server (ends with "0000").
pub fn build_lines_data(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut result = Vec::new();
    for line in lines {
        let len = line.len() + 5;
        let header = format!("{:04x}", len);
        result.extend_from_slice(header.as_bytes());
        result.extend_from_slice(line);
        result.push(b'\n');
    }
    result.extend_from_slice(b"0000");
    result
}

pub fn http_request(
    url: &str,
    username: &str,
    password: &str,
    data: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let mut req = if data.is_some() { ureq::post(url) } else { ureq::get(url) };
    if !username.is_empty() || !password.is_empty() {
        let auth = format!("{}:{}", username, password);
        let encoded = hex::encode(auth);
        req = req.set("Authorization", &format!("Basic {}", encoded));
    }
    let response = if let Some(bytes) = data {
        req.set("Content-Type", "application/x-git-receive-pack-request")
            .send_bytes(bytes)
    } else {
        req.call()
    };
    let res = response.with_context(|| format!("HTTP request failed for URL '{}'", url))?;
    let mut reader = res.into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).with_context(|| format!("failed to read response payload from '{}'", url))?;
    Ok(buf)
}

pub fn get_remote_master_hash(
    git_url: &str,
    username: &str,
    password: &str,
) -> Result<Option<String>> {
    let url = format!("{}/info/refs?service=git-receive-pack", git_url);
    let response = http_request(&url, username, password, None)?;
    let lines = extract_lines(&response);
    if lines.len() < 3 {
        return Ok(None);
    }
    ensure!(lines[0] == b"# service=git-receive-pack\n", "invalid service header in remote response");
    if lines[2].starts_with(b"0000000000000000000000000000000000000000") {
        return Ok(None);
    }
    let first_line = &lines[2];
    let parts: Vec<&[u8]> = first_line.split(|&b| b == b'\0' || b == b' ').collect();
    ensure!(parts.len() >= 2, "failed to parse refs from remote server response");
    let master_sha1 = std::str::from_utf8(parts[0]).context("invalid UTF-8 in remote master SHA-1 string")?;
    let master_ref = std::str::from_utf8(parts[1]).context("invalid UTF-8 in remote ref path name")?;
    ensure!(master_ref == "refs/heads/master", "expected ref 'refs/heads/master', got '{}'", master_ref);
    Ok(Some(master_sha1.to_string()))
}

pub fn push(
    git_url: &str,
    username: Option<String>,
    password: Option<String>,
) -> Result<(Option<String>, std::collections::HashSet<String>)> {
    let user = username.unwrap_or_else(|| std::env::var("GIT_USERNAME").unwrap_or_default());
    let pass = password.unwrap_or_else(|| std::env::var("GIT_PASSWORD").unwrap_or_default());
    let remote_sha1 = get_remote_master_hash(git_url, &user, &pass)?;
    let local_sha1 = crate::commit::get_local_master_hash()?.context("no local commits found to push")?;
    let missing = crate::object::find_missing_objects(&local_sha1, remote_sha1.as_deref())?;
    println!(
        "updating remote master from {} to {} ({} object{})",
        remote_sha1.as_deref().unwrap_or("no commits"),
        local_sha1,
        missing.len(),
        if missing.len() == 1 { "" } else { "s" }
    );
    let zero_sha = "0".repeat(40);
    let ref_command = format!(
        "{} {} refs/heads/master\0 report-status",
        remote_sha1.as_deref().unwrap_or(&zero_sha),
        local_sha1
    );
    let lines = vec![ref_command.as_bytes().to_vec()];
    let mut payload = build_lines_data(&lines);
    let pack_data = crate::object::create_pack(&missing)?;
    payload.extend(pack_data);
    let url = format!("{}/git-receive-pack", git_url);
    let response = http_request(&url, &user, &pass, Some(&payload))?;
    let res_lines = extract_lines(&response);
    ensure!(res_lines.len() >= 2, "expected at least 2 lines in response, got {}", res_lines.len());
    ensure!(res_lines[0] == b"unpack ok\n", "server failed to unpack objects: {:?}", String::from_utf8_lossy(&res_lines[0]));
    ensure!(res_lines[1] == b"ok refs/heads/master\n", "server failed to update ref: {:?}", String::from_utf8_lossy(&res_lines[1]));
    Ok((remote_sha1, missing))
}
