use redis::Value;
use redis::aio::{ConnectionLike, MultiplexedConnection};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

async fn get_connection(address: &str) -> Result<MultiplexedConnection, NanoServiceError> {
    let client = redis::Client::open(address).eject(NanoServiceErrorStatus::Unknown)?;
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;
    Ok(conn)
}

fn unpack_result_string(result: Value) -> Result<String, NanoServiceError> {
    match result {
        Value::SimpleString(s) => Ok(s),
        Value::BulkString(bytes) => String::from_utf8(bytes).map_err(|_| {
            NanoServiceError::new(
                "Redis returned a non-UTF-8 string",
                NanoServiceErrorStatus::Unknown,
            )
        }),
        _ => Err(NanoServiceError::new(
            "Error converting the result into a string",
            NanoServiceErrorStatus::Unknown,
        )),
    }
}

pub async fn login(
    address: &str,
    user_id: &str,
    timeout_mins: usize,
    perm_user_id: i32,
) -> Result<(), NanoServiceError> {
    let mut conn = get_connection(address).await?;
    let result = conn
        .req_packed_command(
            &redis::cmd("session.login")
                .arg(user_id)
                .arg(timeout_mins)
                .arg(perm_user_id.to_string())
                .clone(),
        )
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;

    match result {
        Value::Okay => Ok(()),
        _ => Err(NanoServiceError::new(
            format!("{result:?}"),
            NanoServiceErrorStatus::Unknown,
        )),
    }
}

pub async fn logout(address: &str, user_id: &str) -> Result<String, NanoServiceError> {
    let mut conn = get_connection(address).await?;
    let result = conn
        .req_packed_command(&redis::cmd("session.logout").arg(user_id).clone())
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;
    let result_string = unpack_result_string(result)?;
    Ok(result_string)
}

#[derive(Debug)]
pub enum UserSessionStatus {
    Ok(i32),
    Refresh,
}

pub async fn update(address: &str, user_id: &str) -> Result<UserSessionStatus, NanoServiceError> {
    let mut conn = get_connection(address).await?;
    let result = conn
        .req_packed_command(&redis::cmd("session.update").arg(user_id).clone())
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;

    let result_string = unpack_result_string(result)?;
    match result_string.as_str() {
        "TIMEOUT" => {
            return Err(NanoServiceError::new(
                "Session has timedout",
                NanoServiceErrorStatus::Unauthorized,
            ));
        }
        "NOT_FOUND" => {
            return Err(NanoServiceError::new(
                "Session not found",
                NanoServiceErrorStatus::Unauthorized,
            ));
        }
        "REFRESH" => return Ok(UserSessionStatus::Refresh),
        _ => {}
    }

    let perm_user_id: i32 = result_string.parse().map_err(|_| {
        NanoServiceError::new(
            "Error converting the result into a string",
            NanoServiceErrorStatus::Unknown,
        )
    })?;

    Ok(UserSessionStatus::Ok(perm_user_id))
}
