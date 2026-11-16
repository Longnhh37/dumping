use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{
    schema::{AllTodoItems, ToDoItem},
    transactions::get::{GetAll, GetByName},
};

pub async fn get_all<T: GetAll>(store: &T) -> Result<AllTodoItems, NanoServiceError> {
    let all_items = store.get_all().await?;
    Ok(AllTodoItems::from_vec(all_items))
}

pub async fn get_by_name<T: GetByName>(
    store: &T,
    name: &str,
) -> Result<ToDoItem, NanoServiceError> {
    store.get_by_name(name).await
}

#[cfg(test)]
mod tests {
    use super::{get_all, get_by_name};
    use glue::errors::NanoServiceError;
    use to_do_dal::to_do_items::{
        enums::TaskStatus,
        schema::ToDoItem,
        transactions::get::{GetAll, GetByName},
    };

    struct TestStore;

    impl GetAll for TestStore {
        async fn get_all(&self) -> Result<Vec<ToDoItem>, NanoServiceError> {
            Ok(vec![
                ToDoItem {
                    name: "pending-item".into(),
                    status: TaskStatus::Pending,
                },
                ToDoItem {
                    name: "done-item".into(),
                    status: TaskStatus::Done,
                },
            ])
        }
    }

    impl GetByName for TestStore {
        async fn get_by_name(&self, name: &str) -> Result<ToDoItem, NanoServiceError> {
            Ok(ToDoItem {
                name: name.to_string(),
                status: TaskStatus::Done,
            })
        }
    }

    #[tokio::test]
    async fn get_all_delegates_and_returns_status_groups() {
        let items = get_all(&TestStore).await.unwrap();
        assert_eq!(items.pending[0].name, "pending-item");
        assert_eq!(items.done[0].name, "done-item");
    }

    #[tokio::test]
    async fn get_by_name_forwards_the_name_to_the_store() {
        let item = get_by_name(&TestStore, "requested-name").await.unwrap();
        assert_eq!(item.name, "requested-name");
        assert_eq!(item.status, TaskStatus::Done);
    }
}
