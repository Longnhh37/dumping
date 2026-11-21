use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use to_do_dal::to_do_items::transactions::create::SaveOne;

pub async fn create<T: SaveOne>(
    store: &T,
    item: NewToDoItem,
    user_id: &str,
) -> Result<ToDoItem, NanoServiceError> {
    let created_item = store.save_one(item, user_id).await?;
    Ok(created_item)
}

#[cfg(test)]
mod tests {
    use super::create;
    use glue::errors::NanoServiceError;
    use to_do_dal::to_do_items::{
        enums::TaskStatus,
        schema::{NewToDoItem, ToDoItem},
        transactions::create::SaveOne,
    };

    struct Store;
    impl SaveOne for Store {
        async fn save_one(
            &self,
            item: NewToDoItem,
            user_id: &str,
        ) -> Result<ToDoItem, NanoServiceError> {
            assert_eq!(user_id, "user-1");
            Ok(ToDoItem {
                id: 42,
                title: item.title,
                status: item.status,
            })
        }
    }

    #[tokio::test]
    async fn create_forwards_item_and_owner_to_store() {
        let item = NewToDoItem {
            title: "write tests".into(),
            status: TaskStatus::Pending,
        };
        let saved = create(&Store, item, "user-1").await.unwrap();
        assert_eq!(saved.id, 42);
        assert_eq!(saved.title, "write tests");
    }
}
