use rocket_contrib::Json;
use error::Error;
use schemas::ethics::{Path, Schema};
use schemas::ethics::schema::ETHICA;
use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use models;
use rocket::State;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;

#[get("/v1/ethics/schema")]
pub fn schema() -> Json<Schema> {
    Json(ETHICA)
}

#[put("/v1/ethics/editions/<edition_id>/fragments", data = "<fragment>")]
pub fn edit_fragment(
    fragment: Json<models::Fragment>,
    edition_id: String,
    conn: DbConn,
) -> Result<Json<models::Fragment>, Error> {
    // let path: Path = fragment.fragment_path.parse()?;
    unimplemented!()
}
