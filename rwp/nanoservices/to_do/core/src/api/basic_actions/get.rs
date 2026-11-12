use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{
    schema::{AllTodoItems, ToDoItem},
    transactions::get::{GetAll, GetByName},
};

pub async fn get_all<T: GetAll>() -> Result<AllTodoItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllTodoItems::from_vec(all_items)
}

pub async fn get_by_name<T: GetByName>(name: &str) -> Result<ToDoItem, NanoServiceError> {
    T::get_by_name(name.to_owned()).await
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use super::{get_all, get_by_name};
    use glue::errors::NanoServiceError;
    use to_do_dal::to_do_items::{
        schema::ToDoItem,
        transactions::get::{GetAll, GetByName},
    };

    struct TestStore;

    impl GetAll for TestStore {
        fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
            std::future::ready(Ok(vec![
                ToDoItem {
                    name: "pending-item".into(),
                    status: "PENDING".into(),
                },
                ToDoItem {
                    name: "done-item".into(),
                    status: "DONE".into(),
                },
            ]))
        }
    }

    impl GetByName for TestStore {
        fn get_by_name(
            name: String,
        ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
            std::future::ready(Ok(ToDoItem {
                name,
                status: "DONE".into(),
            }))
        }
    }

    #[tokio::test]
    async fn get_all_delegates_and_returns_status_groups() {
        let items = get_all::<TestStore>().await.unwrap();
        assert_eq!(items.pending[0].name, "pending-item");
        assert_eq!(items.done[0].name, "done-item");
    }

    #[tokio::test]
    async fn get_by_name_forwards_the_name_to_the_store() {
        let item = get_by_name::<TestStore>("requested-name").await.unwrap();
        assert_eq!(item.name, "requested-name");
        assert_eq!(item.status, "DONE");
    }
}
