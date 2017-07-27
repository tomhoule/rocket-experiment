use chrono::{DateTime, Utc};
use models::validation::*;
use uuid::Uuid;
use std::io::prelude::*;
use rocket::data::*;
use rocket::request::Request;
use serde_urlencoded;
use rocket::outcome as o;
use rocket::http::Status;

use db::schema::*;

use models::fields::*;

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

#[derive(Debug, Deserialize)]
#[derive(Insertable)]
#[table_name="editions"]
pub struct EditionNew {
    title: String,
    editor: String,
    year: i32,
    language_code: String,
}

pub struct EditionPatch {
    title: Option<NonEmptyString>,
    editor: Option<String>,
    year: Option<i32>,
    language_code: Option<String>,
}
