use api::error::*;
use diesel::pg::PgConnection;
use models::edition::Edition;
use rocket_contrib::Template;
use rocket::State;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;

#[derive(Serialize)]
struct EditionIndex {
    edition: Edition,
}

#[get("/ethica/<edition_slug>")]
pub fn ethica_index(edition_slug: String, conn: DbConn) -> Result<Template, Error> {
    let edition = Edition::by_slug(&edition_slug, &*conn.inner().get()?)?;
    Ok(Template::render("ethica", EditionIndex { edition }))
}
