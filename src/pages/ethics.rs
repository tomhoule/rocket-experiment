use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use json;

#[get("/ethics")]
pub fn editions_index(flash: Option<Flash<()>>) -> Template {
    let context = if let Some(flash) = flash {
        json!({
            flash.name(): json::from_str::<json::Value>(flash.msg()).unwrap()
        })
    } else {
        json!({})
    };
    Template::render("editions", &context)
}

#[get("/ethics/editions/create")]
pub fn editions_create() -> Template {
    let context = json!({});
    Template::render("editions/new", &context)
}

#[post("/ethics/editions/new")]
pub fn editions_new() -> Flash<Redirect> {
    Flash::success(Redirect::to("/ethics"), "\"Successfully created an edition\"")
}
