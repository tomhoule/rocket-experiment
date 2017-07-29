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
#[derive(Queryable)]
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

impl Edition {
    pub fn all(conn: &PgConnection) -> Result<Vec<Edition>, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        editions.load(conn)
    }

    pub fn by_id(id: Uuid, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        editions.find(id).first(conn)
    }

    pub fn delete(id: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        delete(editions.find(id)).execute(conn)
    }

    pub fn update(id: Uuid, patch: EditionPatch, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        update(editions.find(id)).set(&patch).get_result(conn)
    }
}

impl EditionNew {
    pub fn save(&self, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;

        insert(self).into(editions).get_result(conn)
    }
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[table_name="editions"]
pub struct EditionPatch {
    pub title: Option<String>,
    pub editor: Option<String>,
    pub year: Option<i32>,
    pub language_code: Option<String>,
}
