use tracing::info;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn log_fields() {
    let user = User {
        id: 42,
        name: "Bob".into(),
    };
    let latency_ms = 12.5;

    // '%' use Display, '?' use Debug, plan values use tracing::Value
    info!(user_id = user.id, name = %user.name, latency_ms, "user logged in");
    info!(?user, "full user debug");

    // custom target
    info!(target: "audit", action = "delete", "resource removed");
}
