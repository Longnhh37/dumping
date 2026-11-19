use chrono::{DateTime, NaiveDateTime, Utc};
use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};

pub struct UserSession {
    pub user_id: String,
    pub key: String,
    pub session_datetime: DateTime<Utc>,
}

impl UserSession {
    pub fn from_id(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_owned(),
            key: format!("user_session_{user_id}"),
            session_datetime: Utc::now(),
        }
    }

    pub fn check_timeout(&mut self, ctx: &Context) -> RedisResult {
        todo!()
    }

    pub fn update_last_interacted(&self, ctx: &Context) -> RedisResult {
        todo!()
    }

    pub fn get_counter(&self, ctx: &Context) -> RedisResult {
        todo!()
    }
}
