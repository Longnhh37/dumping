use tokio_uring::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tokio-uring has its own runtime, cannot use #[tokio::main]
    tokio_uring::start(async {
        let file = File::open("/etc/hostname").await?;

        let buf = vec![0u8; 1024];
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res?;

        println!("{}", String::from_utf8_lossy(&buf[..n]));
        file.close().await?;
        Ok(())
    })
}
