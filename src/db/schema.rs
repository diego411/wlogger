table! {
    channels (channel_name) {
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
        score -> Int4,
    }
}

table! {
    users (user_login) {
        user_login -> Varchar,
    }
}

joinable!(messages -> channels (channel));
joinable!(messages -> users (sender_login));

allow_tables_to_appear_in_same_query!(
    channels,
    messages,
    users,
);
