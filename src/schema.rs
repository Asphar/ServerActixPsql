table! {
    interface (id_interface) {
        id_interface -> Int4,
        dns -> Text,
        listen_port -> Int4,
        interface_name -> Text,
        profile_name -> Text,
        id_users -> Int4,
    }
}

table! {
    pubkey (id_key) {
        id_key -> Int4,
        public_key -> Text,
        id_users_p -> Int4,
    }
}

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
        mail -> Text,
        passwd -> Text,
        verified_email -> Bool,
        interface_address -> Text,
        public_key -> Text,
        date_created -> Timestamp,
    }
}

joinable!(interface -> users (id_users));
joinable!(pubkey -> users (id_users_p));
joinable!(session -> users (id_users));

allow_tables_to_appear_in_same_query!(
    interface,
    pubkey,
    session,
    users,
);
