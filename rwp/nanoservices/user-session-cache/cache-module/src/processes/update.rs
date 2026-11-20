use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

use crate::user_session::{FIELD_PERM_USER_ID, SessionStatus, UserSession};

pub fn update(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
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

    ctx.replicate_verbatim();

    match user_session.check_timeout(ctx)? {
        SessionStatus::Timeout => Ok(SessionStatus::Timeout.into()),

        SessionStatus::Refresh => {
            user_session.update_last_interacted(ctx)?;
            Ok(SessionStatus::Refresh.into())
        }

        SessionStatus::Ok => {
            user_session.update_last_interacted(ctx)?;

            let perm_user_id = key
                .hash_get(FIELD_PERM_USER_ID)?
                .ok_or(RedisError::Str("Could not get perm_user_id"))?;

            Ok(RedisValue::SimpleString(perm_user_id.to_string()))
        }
    }
}
