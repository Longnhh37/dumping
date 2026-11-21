use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(
    store: &T,
    id: i32,
    user_id: &str,
) -> Result<(), NanoServiceError> {
    store.delete_one(id, user_id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::delete;
    use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
    use to_do_dal::to_do_items::{schema::ToDoItem, transactions::delete::DeleteOne};

    struct Store;
    impl DeleteOne for Store {
        async fn delete_one(&self, id: i32, user_id: &str) -> Result<ToDoItem, NanoServiceError> {
            if id == 8 && user_id == "user-1" {
                Ok(ToDoItem {
                    id,
                    title: "removed".into(),
                    status: to_do_dal::to_do_items::enums::TaskStatus::Pending,
                })
            } else {
                Err(NanoServiceError::new(
                    "Item not found",
                    NanoServiceErrorStatus::NotFound,
                ))
            }
        }
    }

    #[tokio::test]
    async fn delete_succeeds_and_preserves_store_errors() {
        assert!(delete(&Store, 8, "user-1").await.is_ok());
        let error = delete(&Store, 9, "user-1").await.unwrap_err();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
    }
}
