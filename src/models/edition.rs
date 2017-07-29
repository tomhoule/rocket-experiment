use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel;
use diesel::pg::PgConnection;

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
#[table_name = "editions"]
pub struct EditionNew {
    pub title: String,
    pub editor: String,
    pub year: i32,
    pub language_code: String,
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

    pub fn delete(uuid: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;
        delete(editions.find(uuid)).execute(conn)
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
    pub fn save(&self, conn: &PgConnection) -> Result<Edition, diesel::result::Error> {
        use db::schema::editions::dsl::*;
        use diesel::*;

        insert(self).into(editions).get_result(conn)
    }
}
