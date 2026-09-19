use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

use crate::{
    models::{post::Post, user::User},
    schema::comments,
};

#[derive(Debug, Clone, Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
}
