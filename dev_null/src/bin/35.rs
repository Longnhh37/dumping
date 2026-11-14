use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, primitives::ByteStream};

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    client
        .put_object()
        .bucket("my-bucket")
        .key("hello.txt")
        .body(ByteStream::from_static(b"hello from rust"))
        .send()
        .await?;

    let resp = client.list_objects_v2().bucket("my-bucket").send().await?;
    for obj in resp.contents() {
        println!("{:?}", obj.key());
    }

    Ok(())
}
