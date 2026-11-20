use std::future::Future;

use glue::errors::NanoServiceError;

pub trait ResolveAuthUser: Clone + Send + Sync + 'static {
    fn resolve_owner_key(
        &self,
        unique_id: &str,
    ) -> impl Future<Output = Result<String, NanoServiceError>> + Send;
}

#[cfg(feature = "auth-http")]
#[derive(Clone, Copy, Default)]
pub struct HttpAuthUserResolver;

#[cfg(feature = "auth-http")]
pub async fn resolve_user_http(unique_id: &str) -> Result<String, NanoServiceError> {
    let user = auth_kernel::api::users::get::get_user_by_unique_id_http(unique_id).await?;
    Ok(user.unique_id)
}

#[cfg(feature = "auth-http")]
impl ResolveAuthUser for HttpAuthUserResolver {
    async fn resolve_owner_key(&self, unique_id: &str) -> Result<String, NanoServiceError> {
        resolve_user_http(unique_id).await
    }
}

#[cfg(feature = "auth-core-postgres")]
#[derive(Clone)]
pub struct CorePostgresAuthUserResolver<T>(pub T);

#[cfg(feature = "auth-core-postgres")]
pub async fn resolve_user_core_postgres<T>(
    auth_store: &T,
    unique_id: &str,
) -> Result<String, NanoServiceError>
where
    T: auth_dal::users::transactions::get::GetByUniqueId,
{
    let user =
        auth_kernel::api::users::get::get_user_by_unique_id_core(auth_store, unique_id).await?;
    Ok(user.unique_id)
}

#[cfg(feature = "auth-core-postgres")]
impl<T> ResolveAuthUser for CorePostgresAuthUserResolver<T>
where
    T: auth_dal::users::transactions::get::GetByUniqueId + Clone + Send + Sync + 'static,
{
    async fn resolve_owner_key(&self, unique_id: &str) -> Result<String, NanoServiceError> {
        resolve_user_core_postgres(&self.0, unique_id).await
    }
}
