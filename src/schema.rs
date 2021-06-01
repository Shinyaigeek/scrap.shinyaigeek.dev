table! {
    threads (id) {
        id -> Int4,
        title -> Varchar,
        published -> Bool,
        is_open -> Bool,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        gh_user_id -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    threads,
    users,
);
