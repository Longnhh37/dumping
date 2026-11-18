use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{
    schema::{AllTodoItems, ToDoItem},
    transactions::get::{GetAll, GetById},
};

pub async fn get_all<T: GetAll>(
    store: &T,
    user_id: &str,
) -> Result<AllTodoItems, NanoServiceError> {
    let all_items = store.get_all(user_id).await?;
    Ok(AllTodoItems::from_vec(all_items))
}

pub async fn get_by_id<T: GetById>(
    store: &T,
    id: i32,
    user_id: &str,
) -> Result<ToDoItem, NanoServiceError> {
    store.get_by_id(id, user_id).await
}

#[cfg(test)]
mod tests {
    use super::{get_all, get_by_id};
    use glue::errors::NanoServiceError;
    use to_do_dal::to_do_items::{
        enums::TaskStatus,
        schema::ToDoItem,
        transactions::get::{GetAll, GetById},
    };

    struct TestStore;

    impl GetAll for TestStore {
        async fn get_all(&self, _user_id: &str) -> Result<Vec<ToDoItem>, NanoServiceError> {
            Ok(vec![
                ToDoItem {
                    id: 1,
                    title: "pending-item".into(),
                    status: TaskStatus::Pending,
                },
                ToDoItem {
                    id: 2,
                    title: "done-item".into(),
                    status: TaskStatus::Done,
                },
            ])
        }
    }

    impl GetById for TestStore {
        async fn get_by_id(&self, id: i32, _user_id: &str) -> Result<ToDoItem, NanoServiceError> {
            Ok(ToDoItem {
                id,
                title: format!("item-{id}"),
                status: TaskStatus::Done,
            })
        }
    }

    #[tokio::test]
    async fn get_all_delegates_and_returns_status_groups() {
        let items = get_all(&TestStore, "user-1").await.unwrap();
        assert_eq!(items.pending[0].title, "pending-item");
        assert_eq!(items.done[0].title, "done-item");
    }

    #[tokio::test]
    async fn get_by_id_forwards_the_id_to_the_store() {
        let item = get_by_id(&TestStore, 7, "user-1").await.unwrap();
        assert_eq!(item.id, 7);
        assert_eq!(item.status, TaskStatus::Done);
    }
}
