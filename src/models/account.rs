use chrono::{DateTime, Utc};
use diesel;
use uuid::Uuid;
use db::schema::accounts;

#[derive(Debug, Identifiable, Queryable, Deserialize, Serialize)]
pub struct Account {
    id: Uuid,
    email: String,
    password_hash: String,
    confirmed_at: Option<DateTime<Utc>>,
    admin: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
