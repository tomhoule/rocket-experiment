use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel;
use diesel::pg::PgConnection;
use validator::Validate;

use db::schema::*;

pub static LANGUAGE_CODES: &'static [&'static str] = &[
    "de",
    "en",
    "es",
    "fr",
    "ja",
    "la",
    "ru",
    "zh",
];

#[derive(Queryable, AsPursType, Serialize)]
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

#[derive(Debug, Deserialize, Serialize, Insertable, AsPursType, Validate)]
#[table_name = "editions"]
pub struct EditionNew {
    #[validate(length(message = "Please specify a title", min = "1"))]
    pub title: String,
    #[validate(length(message = "The editor is missing", min = "1"))]
    pub editor: String,
    #[validate(range(message = "Please specify a valid year", min = "1500", max = "3000"))]
    pub year: i32,
    #[validate(length(equal = "2"))]
    pub language_code: String,
    #[validate(length(message = "The edition needs a valid slug", min = "2"))]
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[table_name = "editions"]
pub struct EditionPatch {
    pub title: Option<String>,
    pub editor: Option<String>,
    pub year: Option<i32>,
    pub language_code: Option<String>,
}

impl Edition {
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

    pub fn to_protobuf(self) -> ::rpc::repository::Edition {
        let mut pb = ::rpc::repository::Edition::new();
        pb.set_title(self.title);
        pb.set_slug(self.slug);
        pb.set_editor(self.editor);
        pb.set_year(self.year);
        pb.set_language_code(self.language_code);
        pb.set_created_at(self.created_at.to_rfc3339());
        pb.set_updated_at(self.updated_at.to_rfc3339());
        pb
    }

    pub fn update(
        uuid: Uuid,
        patch: EditionPatch,
        conn: &PgConnection,
    ) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        update(editions.find(uuid)).set(&patch).get_result(conn)
    }
}

impl EditionNew {
    pub fn from_protobuf(pb: ::rpc::repository::Edition) -> EditionNew {
        EditionNew {
            title: pb.title,
            language_code: pb.language_code,
            editor: pb.editor,
            slug: pb.slug,
            year: pb.year,
        }
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;

        insert(self).into(editions).get_result(conn)
    }
}
