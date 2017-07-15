use chrono::{DateTime, Utc};
use uuid::Uuid;

struct Edition
{
    id: Uuid,
    title: String,
    editor: String,
    language_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
