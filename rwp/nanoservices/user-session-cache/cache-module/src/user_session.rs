use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, NaiveDateTime, Utc};
use redis_module::{Context, RedisError, RedisResult, RedisValue, key::RedisKeyWritable};

pub const FIELD_LAST_INTERACTED: &str = "last_interacted";
pub const FIELD_TIMEOUT_MINS: &str = "timeout_mins";
pub const FIELD_COUNTER: &str = "counter";
pub const FIELD_PERM_USER_ID: &str = "perm_user_id";

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const REFRESH_THRESHOLD: i64 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Ok,
    Refresh,
    Timeout,
}

impl From<SessionStatus> for RedisValue {
    fn from(value: SessionStatus) -> Self {
        RedisValue::SimpleStringStatic(match value {
            SessionStatus::Ok => "OK",
            SessionStatus::Refresh => "REFRESH",
            SessionStatus::Timeout => "TIMEOUT",
        })
    }
}

fn parse_field<T>(key: &RedisKeyWritable, field: &str) -> Result<T, RedisError>
where
    T: FromStr,
    T::Err: Display,
{
    let value = key
        .hash_get(field)?
        .ok_or_else(|| RedisError::String(format!("{field} field does not exist")))?;

    value
        .try_as_str()?
        .parse()
        .map_err(|e| RedisError::String(format!("could not parse {field} : {e}")))
}

pub struct UserSession {
    pub user_id: String,
    session_datetime: DateTime<Utc>,
}

impl UserSession {
    pub fn new(user_id: &str, session_datetime: DateTime<Utc>) -> Self {
        Self {
            user_id: user_id.to_owned(),
            session_datetime,
        }
    }

    pub fn from_id(user_id: &str) -> Self {
        Self::new(user_id, Utc::now())
    }

    pub(crate) fn open_key(&self, ctx: &Context) -> RedisKeyWritable {
        let key = format!("user_session_{}", self.user_id);
        ctx.open_key_writable(&ctx.create_string(key))
    }

    pub fn check_timeout(&self, ctx: &Context) -> Result<SessionStatus, RedisError> {
        let key = self.open_key(ctx);

        let last_interacted = {
            let raw: String = parse_field(&key, FIELD_LAST_INTERACTED)?;
            NaiveDateTime::parse_from_str(&raw, DATE_FORMAT).map_err(|e| {
                ctx.log_warning(&format!("could not parse date: {e:?}"));
                RedisError::Str("Could not parse date")
            })?
        };
        let timeout_mins: i64 = parse_field(&key, FIELD_TIMEOUT_MINS)?;

        let elapsed_mins = self
            .session_datetime
            .naive_utc()
            .signed_duration_since(last_interacted)
            .num_minutes();

        if elapsed_mins > timeout_mins {
            key.delete()?;
            return Ok(SessionStatus::Timeout);
        }

        let counter = Self::read_counter(&key)? + 1;
        key.hash_set(FIELD_COUNTER, ctx.create_string(counter.to_string()));

        Ok(if counter > REFRESH_THRESHOLD {
            SessionStatus::Refresh
        } else {
            SessionStatus::Ok
        })
    }

    pub fn update_last_interacted(&self, ctx: &Context) -> RedisResult {
        let key = self.open_key(ctx);
        let formatted = self.session_datetime.format(DATE_FORMAT).to_string();
        key.hash_set(FIELD_LAST_INTERACTED, ctx.create_string(formatted));
        Ok(RedisValue::SimpleStringStatic("OK"))
    }

    fn read_counter(key: &RedisKeyWritable) -> Result<i64, RedisError> {
        parse_field(key, FIELD_COUNTER)
    }
}
