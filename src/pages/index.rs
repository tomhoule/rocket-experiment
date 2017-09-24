use rocket_contrib::Template;
use chrono::*;

#[derive(Serialize)]
pub struct Index {
    now: DateTime<Utc>,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &Index { now: Utc::now() })
}
