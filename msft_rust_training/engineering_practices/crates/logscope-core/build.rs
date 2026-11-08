use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");

    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());

    let timestamp = std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|duration| duration.as_secs().to_string())
        })
        .unwrap_or_else(|| "unknown".to_owned());

    println!("cargo:rustc-env=LOGSCOPE_GIT_SHA={git_sha}");
    println!("cargo:rustc-env=LOGSCOPE_BUILD_TIMESTAMP={timestamp}");
}
