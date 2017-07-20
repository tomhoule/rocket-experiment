use chrono::{DateTime, Utc};
use models::validation::*;
use uuid::Uuid;
use std::io::prelude::*;
use rocket::data::*;
use rocket::request::Request;
use serde_urlencoded;
use rocket::outcome as o;
use rocket::http::Status;

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
pub struct EditionNew {
    title: NonEmptyString,
    editor: String,
    year: i32,
    language_code: i32,
}

pub struct EditionPatch {
    title: Option<NonEmptyString>,
    editor: Option<String>,
    year: Option<i32>,
    language_code: Option<String>,
}

impl FromData for EditionNew {
    type Error = ValidationError;

    fn from_data(request: &Request, data: Data) -> Outcome<Self, Self::Error> {
        match serde_urlencoded::from_reader(data.open()) {
            Ok(edition) => o::Outcome::Success(edition),
            Err(err) => o::Outcome::Failure((Status::BadRequest,  err.into())),
        }
    }
}
