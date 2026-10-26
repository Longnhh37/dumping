use std::time::Duration;

use redis::{AsyncCommands, aio::ConnectionManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

async fn load_from_db(id: u64) -> User {
    tokio::time::sleep(Duration::from_millis(200)).await;
    User { id, name: format!("user_{id}") }
}

async fn get_user(
    redis: &mut ConnectionManager,
    id: u64,
) -> Result<User, Box<dyn std::error::Error>> {
    let key = format!("user:{id}");

    // 1. Check cache
    if let Some(json) = redis.get::<_, Option<String>>(&key).await? {
        println!("cache HIT  {key}");
        return Ok(serde_json::from_str(&json)?);
    }

    // 2. Miss: read DB then write cache with TTL
    println!("cache MISS {key}");
    let user = load_from_db(id).await;
    let ttl = 60 + (id % 10); // small jitter to prevent avalanche
    redis
        .set_ex::<_, _, ()>(&key, serde_json::to_string(&user)?, ttl)
        .await?;
    Ok(user)
}

async fn invalidate_user(
    redis: &mut ConnectionManager,
    id: u64,
) -> redis::RedisResult<()> {
    redis.del::<_, ()>(format!("user:{id}")).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut redis = ConnectionManager::new(client).await?;

    println!("{:?}", get_user(&mut redis, 42).await?); // MISS, ~200ms
    println!("{:?}", get_user(&mut redis, 42).await?); // HIT, <1ms

    invalidate_user(&mut redis, 42).await?;
    Ok(())
}
