table! {
    threads (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        is_open -> Bool,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
