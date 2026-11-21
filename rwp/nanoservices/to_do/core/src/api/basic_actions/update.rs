use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::update::UpdateOne;

pub async fn update<T: UpdateOne>(
    store: &T,
    item: ToDoItem,
    user_id: &str,
) -> Result<(), NanoServiceError> {
    store.update_one(item, user_id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::update;
    use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
    use to_do_dal::to_do_items::{
        enums::TaskStatus, schema::ToDoItem, transactions::update::UpdateOne,
    };

    struct Store;
    impl UpdateOne for Store {
        async fn update_one(
            &self,
            item: ToDoItem,
            user_id: &str,
        ) -> Result<ToDoItem, NanoServiceError> {
            if user_id == "user-1" && item.id == 5 {
                Ok(item)
            } else {
                Err(NanoServiceError::new(
                    "Item not found",
                    NanoServiceErrorStatus::NotFound,
                ))
            }
        }
    }

    #[tokio::test]
    async fn update_succeeds_and_preserves_store_errors() {
        let item = ToDoItem {
            id: 5,
            title: "updated".into(),
            status: TaskStatus::Done,
        };
        assert!(update(&Store, item.clone(), "user-1").await.is_ok());
        let error = update(&Store, item, "other-user").await.unwrap_err();
        assert_eq!(error.status, NanoServiceErrorStatus::NotFound);
    }
}
