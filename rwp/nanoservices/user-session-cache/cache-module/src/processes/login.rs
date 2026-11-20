use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

use crate::user_session::{FIELD_COUNTER, FIELD_PERM_USER_ID, FIELD_TIMEOUT_MINS, UserSession};

pub fn login(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 4 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);

    let user_id = args.next_string()?;
    let timeout_mins = args
        .next_string()?
        .parse::<i64>()
        .map_err(|_| RedisError::String("timeout_mins must be an integer".to_owned()))?;
    if timeout_mins <= 0 {
        return Err(RedisError::String(
            "timeout_mins must be a positive integer".to_owned(),
        ));
    }
    let perm_user_id = args.next_string()?;

    let user_session = UserSession::from_id(&user_id);
    user_session.update_last_interacted(ctx)?;

    let key = user_session.open_key(ctx);
    key.hash_set(
        FIELD_TIMEOUT_MINS,
        ctx.create_string(timeout_mins.to_string()),
    );
    key.hash_set(FIELD_COUNTER, ctx.create_string("0"));
    key.hash_set(FIELD_PERM_USER_ID, ctx.create_string(perm_user_id));

    Ok(RedisValue::SimpleStringStatic("OK"))
}
