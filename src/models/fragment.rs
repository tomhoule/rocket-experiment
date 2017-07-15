use chrono::{DateTime, Utc};
use uuid::Uuid;

struct Fragment {
    edition_id: Uuid,
    path: String,
    editor: String,
    language_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
