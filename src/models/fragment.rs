use error::Error;
use chrono::{DateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::*;
use rpc::repository::EthicsFragment;
use validator::Validate;
use db::schema::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Fragment {
    pub id: Uuid,
    pub edition_id: Uuid,
    pub fragment_path: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, AsChangeset, Insertable)]
#[table_name = "fragments"]
pub struct FragmentPatch {
    #[validate(length(min = "1"))]
    pub fragment_path: String,
    pub edition_id: Uuid,
    #[validate(length(min = "1"))]
    pub value: String,
}

impl FragmentPatch {
    pub fn from_proto(proto: EthicsFragment) -> Result<Self, Error> {
        let patch = FragmentPatch {
            fragment_path: proto.path,
            edition_id: proto.edition_id.parse()?,
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
            .get_result(conn)
    }
}

impl Fragment {
    pub fn for_edition(id: &str, conn: &PgConnection) -> Result<Vec<Fragment>, diesel::result::Error> {
        use db::schema::fragments::dsl::*;
        fragments
            .filter(edition_id.eq(id))
            .load(conn)
    }

    #[deny(unused_variables)]
    pub fn into_proto(self) -> EthicsFragment {
        let Fragment {
            fragment_path,
            edition_id,
            value,
            ..
        } = self;
        let mut proto = EthicsFragment::new();
        proto.set_path(fragment_path);
        proto.set_edition_id(format!("{}", edition_id));
        proto.set_value(value);
        proto
    }
}
