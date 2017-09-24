use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel;
use diesel::pg::PgConnection;
use diesel::*;

#[derive(Queryable, AsPursType, Serialize)]
pub struct Fragment {
    id: Uuid,
    fragment_path: String,
    edition_id: Uuid,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Fragment {
    fn from_edition_id(uuid: Uuid, conn: &PgConnection) -> Result<Vec<Fragment>, diesel::result::Error> {
        use db::schema::fragments::dsl::*;
        fragments
            .filter(edition_id.eq(uuid))
            .load(conn)
    }
}
