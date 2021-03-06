use chrono::{DateTime, Utc};
use uuid::Uuid;
use diesel::{delete, update, insert_into};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use validator::Validate;

use db::schema::*;

#[derive(Identifiable, Queryable, Serialize)]
pub struct Edition {
    pub id: Uuid,
    title: String,
    editor: String,
    year: i32,
    pub language_code: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Validate, FromForm)]
#[table_name = "editions"]
pub struct EditionNew {
    #[validate(length(message = "Please specify a title", min = "1"))] pub title: String,
    #[validate(length(message = "The editor is missing", min = "1"))] pub editor: String,
    #[validate(range(message = "Please specify a valid year", min = "1500", max = "3000"))]
    pub year: i32,
    #[validate(length(equal = "2"))] pub language_code: String,
    #[validate(length(message = "The edition needs a valid slug", min = "2"))] pub slug: String,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset, FromForm, Validate)]
#[table_name = "editions"]
pub struct EditionPatch {
    pub title: Option<String>,
    pub editor: Option<String>,
    pub slug: Option<String>,
    pub year: Option<i32>,
    pub language_code: Option<String>,
}

// macro_rules! take {
//     ($proto:ident, $field_name:ident, $default:expr) => {
//         if $proto.$field_name == $default {
//             None
//         } else {
//             Some($proto.$field_name)
//         }
//     }
// }

impl EditionPatch {
    // pub fn from_proto(proto: rpc::Edition) -> Self {
    //     EditionPatch {
    //         title: take!(proto, title, ""),
    //         editor: take!(proto, editor, ""),
    //         slug: None,
    //         year: take!(proto, year, 0),
    //         language_code: take!(proto, language_code, ""),
    //     }
    // }

    pub fn save(&self, id_: Uuid, conn: &PgConnection) -> QueryResult<Edition> {
        use db::schema::editions::dsl::*;
        update(editions.filter(id.eq(id_)))
            .set(self)
            .get_result(conn)
    }
}

impl Edition {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Edition>> {
        use db::schema::editions::dsl::*;
        editions.load(conn)
    }

    pub fn by_id(uuid: Uuid, conn: &PgConnection) -> QueryResult<Edition> {
        use db::schema::editions::dsl::*;
        editions.find(uuid).first(conn)
    }

    pub fn by_slug(req_slug: &str, conn: &PgConnection) -> QueryResult<Edition> {
        use db::schema::editions::dsl::*;
        editions.filter(slug.eq(req_slug)).first(conn)
    }

    pub fn by_slugs<'a, T: Iterator<Item = &'a str>>(
        req_slug: T,
        conn: &PgConnection,
    ) -> QueryResult<Vec<Edition>> {
        use db::schema::editions::dsl::*;
        editions.filter(slug.eq_any(req_slug)).load(conn)
    }

    pub fn delete(uuid: Uuid, conn: &PgConnection) -> QueryResult<usize> {
        use db::schema::editions::dsl::*;
        delete(editions.find(uuid)).execute(conn)
    }
}

impl EditionNew {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<Edition> {
        use db::schema::editions::dsl::*;
        insert_into(editions).values(self).get_result(conn)
    }
}
