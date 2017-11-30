table! {
    accounts (id) {
        id -> Uuid,
        email -> Text,
        password_hash -> Text,
        confirmed_at -> Nullable<Timestamptz>,
        admin -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    editions (id) {
        id -> Uuid,
        title -> Text,
        editor -> Text,
        year -> Int4,
        language_code -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        slug -> Varchar,
    }
}

table! {
    fragments (id) {
        id -> Uuid,
        edition_id -> Uuid,
        fragment_path -> Text,
        value -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(fragments -> editions (edition_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    editions,
    fragments,
);
