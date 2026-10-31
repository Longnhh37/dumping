use tracing::{info, info_span, instrument};

// automatically create a span named "fetch_user" with 'id' recorded as field
#[instrument]
fn fetch_user(id: u64) -> String {
    info!("querying db");
    format!("user-{id}")
}

// skip noisy args, add custom fields, and log errors automatically
#[instrument(skip(password), fields(request_id = %uuid_like()), err)]
fn login(username: &str, password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("empty password".into());
    }
    info!("login ok");
    Ok(())
}

fn uuid_like() -> u32 {
    7
}

fn manual_span() {
    // Manual span: everything inside the guard scope is attached to it
    let span = info_span!("process_batch", batch_id = 1);
    let _guard = span.enter();
    info!("processing");
}
