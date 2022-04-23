table! {
    comments (id) {
        id -> Int4,
        author -> Nullable<Varchar>,
        content -> Text,
        thread -> Int4,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rel_tag_thread (id) {
        id -> Int4,
        tag -> Int4,
        thread -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag -> Text,
        icon -> Text,
        create_at -> Timestamp,
    }
}

table! {
    threads (id) {
        id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        is_open -> Bool,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (gh_user_id) {
        gh_user_id -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    rel_tag_thread,
    tags,
    threads,
    users,
);
