use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable, AsPursType, Serialize)]
struct Fragment {
    id: Uuid,
    edition_id: Uuid,
    fragment_path: String,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
