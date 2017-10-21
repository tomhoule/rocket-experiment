use error::Error;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::*;
use rpc::repository::EthicsFragment;
use validator::Validate;
use db::schema::*;

#[derive(Queryable)]
pub struct Fragment {
    pub fragment_path: String,
    pub edition_slug: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, AsChangeset, Insertable)]
#[table_name = "fragments"]
pub struct FragmentPatch {
    #[validate(length(min = "1"))]
    pub fragment_path: String,
    #[validate(length(min = "1"))]
    pub edition_slug: String,
    #[validate(length(min = "1"))]
    pub value: String,
}

impl FragmentPatch {
    pub fn from_proto(proto: EthicsFragment) -> Result<Self, Error> {
        let patch = FragmentPatch {
            fragment_path: proto.path,
            edition_slug: proto.edition_slug,
            value: proto.value,
        };
        patch.validate()?;
        Ok(patch)
    }

    pub fn save(&self, conn: &PgConnection) -> Result<Fragment, diesel::result::Error> {
        use db::schema::fragments::dsl::*;
        // use diesel::pg::upsert::*;
        insert(self).into(fragments)
            // .values(self)
            // .on_conflict(())
            // .do_update()
            // .set(value.eq(self.value))
            .execute(conn);
        unimplemented!()
    }
}

impl Fragment {
    pub fn for_edition(slug: &str, conn: &PgConnection) -> Result<Vec<Fragment>, diesel::result::Error> {
        use db::schema::fragments::dsl::*;
        fragments
            .filter(edition_slug.eq(slug))
            .load(conn)
    }

    #[deny(unused_variables)]
    pub fn into_proto(self) -> EthicsFragment {
        let Fragment {
            fragment_path,
            edition_slug,
            value,
            ..
        } = self;
        let mut proto = EthicsFragment::new();
        proto.set_path(fragment_path);
        proto.set_edition_slug(edition_slug);
        proto.set_value(value);
        proto
    }
}
