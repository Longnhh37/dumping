use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

use crate::{models::user::User, schema::posts};

#[derive(Debug, Clone, Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub body: String,
}
