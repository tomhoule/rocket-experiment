use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel;
use diesel::pg::PgConnection;
use validator::Validate;
use rpc::repository as rpc;
use error;

use db::schema::*;

#[derive(Queryable, Serialize)]
pub struct Edition {
    id: Uuid,
    title: String,
    editor: String,
    year: i32,
    language_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    slug: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Validate)]
#[table_name = "editions"]
pub struct EditionNew {
    #[validate(length(message = "Please specify a title", min = "1"))] pub title: String,
    #[validate(length(message = "The editor is missing", min = "1"))] pub editor: String,
    #[validate(range(message = "Please specify a valid year", min = "1500", max = "3000"))]
    pub year: i32,
    #[validate(length(equal = "2"))] pub language_code: String,
    #[validate(length(message = "The edition needs a valid slug", min = "2"))] pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "editions"]
pub struct EditionPatch {
    pub title: Option<String>,
    pub editor: Option<String>,
    pub year: Option<i32>,
    pub language_code: Option<String>,
}

macro_rules! take {
    ($proto:ident, $field_name:ident, $default:expr) => {
        if $proto.$field_name == $default {
            None
        } else {
            Some($proto.$field_name)
        }
    }
}

impl EditionPatch {
    pub fn from_proto(proto: rpc::Edition) -> Self {
        EditionPatch {
            title: take!(proto, title, ""),
            editor: take!(proto, editor, ""),
            year: take!(proto, year, 0),
            language_code: take!(proto, language_code, ""),
        }
    }

    pub fn save(&self, id_: Uuid, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        update(editions.filter(id.eq(id_)))
            .set(self)
            .get_result(conn)
    }
}

impl Edition {
    #[deny(unused_variables)]
    pub fn into_proto(self) -> rpc::Edition {
        let mut edition = rpc::Edition::new();
        let Edition {
            id: _unused_id,
            year,
            editor,
            title,
            slug,
            updated_at,
            created_at,
            language_code,
        } = self;
        edition.set_year(year);
        edition.set_editor(editor);
        edition.set_title(title);
        edition.set_slug(slug);
        edition.set_updated_at(updated_at.to_rfc3339());
        edition.set_created_at(created_at.to_rfc3339());
        edition.set_language_code(language_code);
        edition
    }

    pub fn all(conn: &PgConnection) -> Result<Vec<Edition>, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        editions.load(conn)
    }

    pub fn by_id(uuid: Uuid, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        editions.find(uuid).first(conn)
    }

    pub fn by_slug(req_slug: &str, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        editions.filter(slug.eq(req_slug)).first(conn)
    }

    pub fn delete(uuid: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        delete(editions.find(uuid)).execute(conn)
    }
}

impl EditionNew {
    pub fn from_proto(proto: rpc::Edition) -> Result<Self, error::Error> {
        let payload = EditionNew {
            title: proto.title,
            editor: proto.editor,
            year: proto.year,
            slug: proto.slug,
            language_code: proto.language_code,
        };
        payload.validate()?;
        Ok(payload)
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;

        insert(self).into(editions).get_result(conn)
    }
}
