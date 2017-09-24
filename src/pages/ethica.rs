use api::error::*;
use diesel::pg::PgConnection;
use models::edition::{Edition, EditionNew, LANGUAGE_CODES};
use rocket_contrib::Template;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use validator::Validate;
use rocket::State;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;

#[derive(Serialize)]
struct EditionIndex {
    edition: Edition,
    success: Option<String>,
}

#[get("/ethica/<edition_slug>")]
pub fn ethica_index(edition_slug: String, conn: DbConn, flash: Option<FlashMessage>) -> Result<Template, Error> {
    let success = flash.and_then(|f| extract_flash(f, "success"));
    let edition = Edition::by_slug(&edition_slug, &*conn.inner().get()?)?;
    Ok(Template::render("ethica", EditionIndex { edition, success }))
}

#[derive(Serialize)]
struct EditionNewVars {
    error: Option<::json::Value>,
    language_codes: &'static [&'static str],
}

fn extract_flash(flash: FlashMessage, name: &'static str) -> Option<String> {
    if flash.name() == name {
        Some(flash.msg().to_string())
    } else {
        None
    }
}

#[get("/ethica/new")]
pub fn ethica_new(flash: Option<FlashMessage>) -> Result<Template, Error> {
    let error = if let Some(error) = flash.and_then(|f| extract_flash(f, "error")) {
        ::json::from_str(&error).unwrap_or(None)
    } else {
        None
    };

    Ok(Template::render("editions/new", EditionNewVars {
        error,
        language_codes: LANGUAGE_CODES,
    }))
}

fn error_list(errors: ::validator::ValidationErrors) -> Vec<String> {
    let inner = errors.inner();
    inner
        .values()
        .flat_map(|errs|
            errs
                .iter()
                .map(|err| {
                    // TODO: default to the field name when there is error message
                    format!("{}", err.message.as_ref().map(|m| m.as_ref()).unwrap_or("".into()))
                }))
        .collect()
}

#[post("/ethica/create", data = "<edition>")]
pub fn ethica_create(edition: Form<EditionNew>, conn: DbConn) -> Result<Flash<Redirect>, Error> {
    Ok(match edition.get().validate() {
        Ok(_) => {
            edition.get().save(&*conn.inner().get()?)?;
            // TODO: Migrate that to new type-safe urls
            let slug = &edition.get().slug;
            Flash::success(Redirect::to(&format!("/ethica/{}", slug)), format!("Congrats on your edition!"))
        },
        Err(err) => Flash::error(Redirect::to("/ethica/new"), ::json::to_string(&error_list(err))?)
    })
}
