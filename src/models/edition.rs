use chrono::{DateTime, Utc};
use models::validation::*;
use uuid::Uuid;
use std::io::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use rocket::data::*;
use rocket::request::Request;
use serde_urlencoded;
use rocket::outcome as o;
use rocket::http::Status;

use models::fields::*;
use db::schema::*;

#[derive(Serialize)]
pub struct Edition {
    id: Uuid,
    title: String,
    editor: String,
    year: i32,
    language_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name="editions"]
pub struct EditionNew {
    pub title: String,
    pub editor: String,
    pub year: i32,
    pub language_code: String,
}

impl EditionNew {
    pub fn save(&self, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;

        insert(self).into(editions).execute(conn)
    }
}

pub struct EditionPatch {
    title: Option<NonEmptyString>,
    editor: Option<String>,
    year: Option<i32>,
    language_code: Option<String>,
}
