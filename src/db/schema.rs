table! {
    channels (id) {
        id -> Int4,
        channel_name -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        channel -> Varchar,
        sender_login -> Varchar,
        post_timestamp -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    messages,
);
