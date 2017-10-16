use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel;
use diesel::pg::PgConnection;
use validator::Validate;

use db::schema::*;

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

    #[deny(unused_variables)]
    pub fn to_proto(self) -> ::rpc::repository::Edition {
        let mut pb = ::rpc::repository::Edition::new();
        let Edition {
            created_at,
            editor,
            id: _unused_id,
            language_code,
            slug,
            title,
            updated_at,
            year,
        } = self;
        pb.set_title(title);
        pb.set_slug(slug);
        pb.set_editor(editor);
        pb.set_year(year);
        pb.set_language_code(language_code);
        pb.set_created_at(created_at.to_rfc3339());
        pb.set_updated_at(updated_at.to_rfc3339());
        pb
    }
}
