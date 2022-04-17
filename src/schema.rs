table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        channel -> Varchar,
        sender_login -> Varchar,
        post_timestamp -> Int4,
    }
}
