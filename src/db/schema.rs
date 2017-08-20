table! {
    editions (id) {
        id -> Uuid,
        title -> Text,
        editor -> Text,
        year -> Int4,
        language_code -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    fragments (id) {
        id -> Uuid,
        fragment_path -> Text,
        edition_id -> Uuid,
        value -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
