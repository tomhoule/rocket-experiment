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
