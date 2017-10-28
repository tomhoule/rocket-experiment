use models;

use api::error::*;
use diesel::pg::PgConnection;
use rocket::State;
use rocket_contrib::Json;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::response::status;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;

#[post("/api/editions", data = "<edition>")]
pub fn editions_create(
    edition: Json<models::EditionNew>,
    conn: DbConn,
) -> Result<status::Created<Json<models::Edition>>, Error> {
    let edition = edition.into_inner().save(&*conn.inner().get()?)?;
    Ok(status::Created("".to_string(), Some(Json(edition))))
}

#[get("/api/editions")]
pub fn editions_index(conn: DbConn) -> Result<Json<Vec<models::Edition>>, Error> {
    Ok(Json(models::Edition::all(&*conn.inner().get()?)?))
}

#[get("/api/editions/<id>")]
pub fn edition(id: String, conn: DbConn) -> Result<Json<models::Edition>, Error> {
    Ok(Json(
        models::Edition::by_id(id.parse()?, &*conn.inner().get()?)?,
    ))
}

#[delete("/api/editions/<id>")]
pub fn edition_delete(id: String, conn: DbConn) -> Result<(), Error> {
    let _deleted = models::Edition::delete(id.parse()?, &*conn.inner().get()?)?;
    Ok(())
}

#[patch("/api/editions/<id>", data = "<patch>")]
pub fn edition_patch(
    patch: Json<models::EditionPatch>,
    id: String,
    conn: DbConn,
) -> Result<Json<models::Edition>, Error> {
    let edition = patch.into_inner().save(id.parse()?, &*conn.inner().get()?)?;
    Ok(Json(edition))
}
