#[cfg(any(feature = "auth-core", feature = "http"))]
mod common_imports {
    pub use auth_dal::users::schema::TrimmedUser;
    pub use glue::errors::NanoServiceError;
}

#[cfg(feature = "auth-core")]
mod core_imports {
    pub use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
    pub use auth_dal::users::descriptors::SqlxPostgresDescriptor;
}

#[cfg(feature = "http")]
mod reqwest_imports {
    pub use glue::{errors::NanoServiceError, token::HeaderToken};
    pub use reqwest::Client;
}

#[cfg(any(feature = "core-postgres", feature = "http"))]
use common_imports::*;

#[cfg(feature = "core-postgres")]
use core_imports::*;

#[cfg(feature = "http")]
use reqwest_imports::*;
