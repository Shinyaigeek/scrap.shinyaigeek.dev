table! {
    comments (id) {
        id -> Int4,
        author -> Varchar,
        content -> Text,
        thread -> Int4,
        published_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
