table! {
    session (id_session) {
        id_session -> Int4,
        uid -> Text,
        id_users -> Int4,
        timestamp -> Timestamp,
    }
}

table! {
    users (id_user) {
        id_user -> Int4,
        username -> Text,
        passwd -> Text,
        date_created -> Timestamp,
    }
}

joinable!(session -> users (id_users));

allow_tables_to_appear_in_same_query!(
    session,
    users,
);
