use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://api.example.com/users/1")
        .header("token", "jwt")
        .send()
        .await?;

    // reqwest does not error on 4xx/5xx, so we check the status ourselves
    let expected = 200;
    if response.status().as_u16() == expected {
        let user: User = response.json().await?;
        println!("{user:?}");
    } else {
        println!("expected {expected}, got {}", response.status());
    }

    Ok(())
}
