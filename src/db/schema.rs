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
    fragments (edition_slug, fragment_path) {
        edition_slug -> Varchar,
        fragment_path -> Text,
        value -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
