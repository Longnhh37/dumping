pub mod api;
mod auth_user;

#[cfg(all(feature = "auth-http", feature = "auth-core-postgres"))]
compile_error!("enable exactly one of `auth-http` or `auth-core-postgres`");

#[cfg(not(any(feature = "auth-http", feature = "auth-core-postgres")))]
compile_error!("enable either `auth-http` or `auth-core-postgres`");
