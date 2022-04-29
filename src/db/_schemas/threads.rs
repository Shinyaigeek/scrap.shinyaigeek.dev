table! {
    threads (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        primary_tag_id -> Int4,
        content -> Text,
        published -> Bool,
        is_open -> Bool,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
