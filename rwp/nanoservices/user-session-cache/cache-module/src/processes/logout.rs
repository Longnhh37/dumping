use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

use crate::user_session::UserSession;

pub fn logout(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);

    let user_id = args.next_string()?;
    let user_session = UserSession::from_id(&user_id);

    let key = user_session.open_key(ctx);
    if key.is_empty() {
        return Ok(RedisValue::SimpleStringStatic("NOT_FOUND"));
    }
    key.delete()?;

    Ok(RedisValue::SimpleStringStatic("OK"))
}
