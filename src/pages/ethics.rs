use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use json;
use models::edition::{Edition, EditionNew};
use validator::Validate;
use error::{Error, validation_errors_to_json};
use super::DbConn;

#[get("/ethics/editions/<slug>")]
pub fn ethics_home(slug: String) -> Template {
    let context = json!({
        "slug": slug,
        "schema": ::schemas::ethics::ETHICA
    });
    Template::render("editions/home", context)
}

#[get("/ethics")]
pub fn editions_index(flash: Option<Flash<()>>, conn: DbConn) -> Result<Template, Error> {
    let mut context = json::Map::new();
    let editions = Edition::all(&*conn.inner().get()?)?;
    if let Some(flash) = flash {
        context.insert(flash.name().into(), json!(flash.msg()));
    }
    context.insert("editions".into(), json!(editions));
    context.insert("links".into(), json!({
        "create_edition": "/ethics/editions/create"
    }));
    Ok(Template::render("editions", &json::Value::Object(context)))
}

#[get("/ethics/editions/create")]
pub fn editions_create(flash: Option<Flash<()>>) -> Template {
    let flash = flash
        .map(|f| (f.name().to_string(), json::from_str::<json::Value>(f.msg())))
        .map(|(name, value)| json!({ name: value.unwrap_or(json::Value::Null) }))
        .unwrap_or(json::Value::Null);
    let context = json!({
        "language_codes": ["fr", "de", "la"],
        "flash": flash
    });
    Template::render("editions/new", &context)
}

#[post("/ethics/editions/new", data = "<form>")]
pub fn editions_new(form: Form<EditionNew>) -> Result<Flash<Redirect>, Error> {
    let payload = form.into_inner();
    match payload.validate() {
        Ok(()) => Ok(Flash::success(Redirect::to("/ethics"), r##""Successfully created an edition""##)),
        Err(errors) => {
            let mut json_err = json::Map::new();
            json_err.insert("original".to_string(), json::to_value(&payload)?);
            json_err.insert("errors".to_string(), validation_errors_to_json(errors));
            let json_err = json::to_string(&json_err).unwrap();
            Ok(Flash::error(Redirect::to("/ethics/editions/create"), json_err))
        },
    }
}
