use chrono::{DateTime, Utc};
use diesel::{insert_into};
use diesel::prelude::*;
use validator::Validate;
use uuid::Uuid;
use models::Edition;
use db::schema::fragments;

#[derive(Debug, Identifiable, Queryable, Deserialize, Serialize, Associations)]
#[belongs_to(Edition, foreign_key = "edition_id")]
pub struct Fragment {
    pub id: Uuid,
    pub edition_id: Uuid,
    pub fragment_path: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, AsChangeset, Insertable)]
#[table_name = "fragments"]
pub struct FragmentPatch {
    #[validate(length(message = "Invalid path", min = "1"))] pub fragment_path: String,
    pub edition_id: Uuid,
    pub value: String,
}

impl FragmentPatch {
    pub fn save(&self, conn: &PgConnection) -> QueryResult<Fragment> {
        use db::schema::fragments::dsl::*;
        use diesel::pg::upsert::*;
        insert_into(fragments)
            .values(self)
            .on_conflict(on_constraint("fragment_uniqueness"))
            .do_update()
            .set(value.eq(excluded(value)))
            .get_result(conn)
    }
}

#[cfg(test)]
mod tests {
    use diesel;
    use diesel::prelude::*;
    use super::*;
    use models::edition::*;
    use dotenv;
    use std::env;

    #[test]
    fn fragment_upsert_works() {
        use db::schema::editions;

        dotenv::dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("database url")
            .to_string();
        let conn = PgConnection::establish(&database_url).expect("Database is up");

        diesel::delete(fragments::table).execute(&conn).unwrap();
        diesel::delete(editions::table).execute(&conn).unwrap();
        let edition = EditionNew {
            title: "".to_string(),
            editor: "".to_string(),
            year: 2000,
            language_code: "de".to_string(),
            slug: "abc".to_string(),
        };
        let edition = edition.save(&conn).unwrap();
        let edition_id = edition.id.clone();
        let patch = FragmentPatch {
            edition_id,
            fragment_path: "not important".to_string(),
            value: "abc".to_string(),
        };
        patch.save(&conn).unwrap();

        let repatch = FragmentPatch {
            edition_id,
            fragment_path: "not important".to_string(),
            value: "defg".to_string(),
        };
        repatch.save(&conn).unwrap();

        let fragments: Vec<Fragment> = Fragment::belonging_to(&edition)
            .load(&conn)
            .unwrap();
        assert_eq!(fragments.len(), 1);
        assert_eq!(fragments[0].value, "defg");
    }
}
