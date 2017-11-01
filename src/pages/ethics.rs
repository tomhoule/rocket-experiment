use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use json;
use models::edition::EditionNew;
use validator::Validate;

#[get("/ethics")]
pub fn editions_index(flash: Option<Flash<()>>) -> Template {
    let mut context = json::Map::new();
    if let Some(flash) = flash {
        context.insert(flash.name().into(), json::from_str::<json::Value>(flash.msg()).unwrap());
    }
    Template::render("editions", &json::Value::Object(context))
}

#[get("/ethics/editions/create")]
pub fn editions_create() -> Template {
    let context = json!({
        "language_codes": ["fr", "de", "la"]
    });
    Template::render("editions/new", &context)
}

#[post("/ethics/editions/new", data = "<form>")]
pub fn editions_new(form: Form<EditionNew>) -> Flash<Redirect> {
    match form.into_inner().validate() {
        Ok(_) => Flash::success(Redirect::to("/ethics"), r##""Successfully created an edition""##),
        Err(error) => Flash::error(Redirect::to("/ethics/editions/create"), r##""Aw, shite""##)
    }
}
