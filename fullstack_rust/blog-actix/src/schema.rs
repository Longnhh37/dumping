// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Integer,
        user_id -> Integer,
        post_id -> Integer,
        body -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(comments, posts, users,);
