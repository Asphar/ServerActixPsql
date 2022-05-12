table! {
    session (uid) {
        uid -> Text,
        id_user -> Int4,
        date_created -> Text,
    }
}

table! {
    users (id_user) {
        id_user -> Int4,
        username -> Text,
        passwd -> Text,
        date_created -> Text,
    }
}

joinable!(session -> users (id_user));

allow_tables_to_appear_in_same_query!(
    session,
    users,
);
