table! {
    editions (id) {
        id -> Uuid,
        title -> Text,
        editor -> Text,
        year -> Int4,
        language_code -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
