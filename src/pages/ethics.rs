use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use json;
use models::edition::{Edition, EditionNew};
use models::fragment::Fragment;
use validator::Validate;
use error::{validation_errors_to_json, Error};
use super::DbConn;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

#[get("/ethics/editions/<edition_slug>/fragments/<fragment_path>")]
pub fn ethics_fragment(edition_slug: String, fragment_path: String, conn: DbConn) -> Result<Template, Error> {
    use diesel::*;
    let conn = &*conn.inner().get()?;
    let edition = Edition::by_slug(&edition_slug, &conn)?;
    let fragments: Vec<Fragment> = Fragment::belonging_to(&edition).load(conn)?;
    unimplemented!();
}

#[get("/ethics/editions/<slug>")]
pub fn ethics_home(slug: String, conn: DbConn) -> Result<Template, Error> {
    use diesel::*;
    let conn = &*conn.inner().get()?;
    let edition = Edition::by_slug(&slug, &conn)?;
    let fragments: Vec<Fragment> = Fragment::belonging_to(&edition).load(conn)?;
    let schema: Vec<json::Value> = ::schemas::ethics::ETHICA.expand().into_iter().map(|exp| {
        let url = format!(
            "/ethics/editions/{}/fragments/{}",
            edition.slug,
            percent_encode(exp.path.as_bytes(), PATH_SEGMENT_ENCODE_SET)
        );
        json!({
            "expanded_node": exp,
            "fragment_url": url
        })
    }).collect();
    // let fragments: Vec<json::Value> = fragments.into_iter().map(|fragment| {
    //     let url = format!(
    //         "/ethics/editions/{}/fragments/{}",
    //         edition.slug,
    //         percent_encode(fragment.fragment_path.as_bytes(), PATH_SEGMENT_ENCODE_SET)
    //     );
    //     json!({
    //         "fragment": fragment,
    //         "url": url
    //     })
    // }).collect();
    let context = json!({
        "slug": slug,
        "schema": schema,
        "fragments": fragments,
    });
    Ok(Template::render("editions/home", context))
}

#[get("/ethics")]
pub fn editions_index(flash: Option<Flash<()>>, conn: DbConn) -> Result<Template, Error> {
    let mut context = json::Map::new();
    let editions = Edition::all(&*conn.inner().get()?)?;
    if let Some(flash) = flash {
        context.insert(flash.name().into(), json!(flash.msg()));
    }
    context.insert("editions".into(), json!(editions));
    context.insert(
        "links".into(),
        json!({
        "create_edition": "/ethics/editions/create"
    }),
    );
    Ok(Template::render("editions", &json::Value::Object(context)))
}

#[get("/ethics/editions/create")]
pub fn editions_create(flash: Option<Flash<()>>) -> Template {
    let flash = flash
        .map(|f| {
            (f.name().to_string(), json::from_str::<json::Value>(f.msg()))
        })
        .map(|(name, value)| json!({ name: value.unwrap_or(json::Value::Null) }))
        .unwrap_or(json::Value::Null);
    let context = json!({
        "language_codes": ["fr", "de", "la"],
        "flash": flash
    });
    Template::render("editions/new", &context)
}

#[get("/ethics/editions/<slug>/edit")]
pub fn editions_edit(slug: String, conn: DbConn) -> Result<Template, Error> {
    let edition = Edition::by_slug(&slug, &*conn.inner().get()?)?;
    let context = json!({
        "edition": edition
    });
    Ok(Template::render("editions/edit", &context))
}

#[post("/ethics/editions/new", data = "<form>")]
pub fn editions_new(form: Form<EditionNew>, conn: DbConn) -> Result<Flash<Redirect>, Error> {
    let payload = form.into_inner();
    match payload.validate() {
        Ok(()) => {
            payload.save(&*conn.inner().get()?)?;
            Ok(Flash::success(
                Redirect::to("/ethics"),
                r##""Successfully created an edition""##,
            ))
        }
        Err(errors) => {
            let mut json_err = json::Map::new();
            json_err.insert("original".to_string(), json::to_value(&payload)?);
            json_err.insert("errors".to_string(), validation_errors_to_json(errors));
            let json_err = json::to_string(&json_err).unwrap();
            Ok(Flash::error(
                Redirect::to("/ethics/editions/create"),
                json_err,
            ))
        }
    }
}
