use tokio::io::AsyncBufReadExt;

async fn count_non_empty_line<R: tokio::io::AsyncBufRead + Unpin>(
    reader: R,
) -> tokio::io::Result<usize> {
    let mut lines = reader.lines();
    let mut cnt = 0;
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            cnt += 1;
        }
    }
    Ok(cnt)
}

// Works with any AsyncBufRead:
// let file = tokio::io::BufReader::new(tokio::fs::File::open("data.txt").await?);
// let count = count_non_empty_lines(file).await?;
//
// let tcp = tokio::io::BufReader::new(TcpStream::connect("...").await?);
// let count = count_non_empty_lines(tcp).await?;
fn main() {}
