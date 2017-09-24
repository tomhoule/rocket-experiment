use api::error::*;
use diesel::pg::PgConnection;
use models::edition::{Edition, LANGUAGE_CODES};
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

#[derive(Serialize)]
struct EditionNewVars {
    language_codes: &'static [&'static str],
}

#[get("/ethica/new")]
pub fn ethica_new() -> Template {
    Template::render("editions/new", EditionNewVars {
        language_codes: LANGUAGE_CODES,
    })
}

#[post("/ethica/create")]
pub fn ethica_create() -> () {
    unimplemented!();
}
