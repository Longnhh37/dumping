mod processes;
mod user_session;

use redis_module::redis_module;

use processes::{login::login, logout::logout, update::update};

redis_module! {
    name: "user_sessions",
    version: 1,
    allocator: (
        redis_module::alloc::RedisAlloc,
        redis_module::alloc::RedisAlloc
    ),
    data_types: [],
    commands: [
        ["sessions.login", login, "write fast deny-oom", 1, 1, 1, ""],
        ["sessions.logout", logout, "write fast deny-oom", 1, 1, 1, ""],
        ["sessions.update", update, "write fast deny-oom", 1, 1, 1, ""],
    ]
}
