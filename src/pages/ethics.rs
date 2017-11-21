use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use json;
use models::edition::{Edition, EditionNew};
use models::fragment::{Fragment, FragmentPatch};
use schemas::ethics::{Path, ETHICS};
use validator::Validate;
use error::{pack_errs, validation_errors_to_json};
use super::DbConn;
use percent_encoding::{percent_encode, PercentEncode, PATH_SEGMENT_ENCODE_SET};
use pages::fail::Failure;

#[derive(Debug, FromForm)]
pub struct FragmentEdit {
    value: String,
}

fn encode_path(path: &[u8]) -> PercentEncode<PATH_SEGMENT_ENCODE_SET> {
    percent_encode(path, PATH_SEGMENT_ENCODE_SET)
}

fn unwrap_flash(flash: Flash<()>) -> json::Value {
    let msg = json::from_str::<json::Value>(flash.msg()).unwrap_or_else(|_| json!(flash.msg()));
    json!({ flash.name(): msg })
}

#[post("/ethics/editions/<edition_slug>/fragments/<path>", data = "<patch>")]
pub fn put_ethics_fragment(
    edition_slug: String,
    path: String,
    patch: Form<FragmentEdit>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Failure> {
    let conn = &*conn.inner().get()?;
    let edition = Edition::by_slug(&edition_slug, &conn)?;
    let patch = patch.into_inner();
    let to = &format!(
        "/ethics/editions/{}/fragments/{}",
        &edition_slug,
        encode_path(&path.as_bytes())
    );

    let parsed_path = match path.parse::<Path>() {
        Ok(p) => {
            if !ETHICS.contains_path(&p) {
                return Ok(Flash::error(Redirect::to(to), "Wrong path"))
            }
            p
        },
        Err(_) => return Ok(Flash::error(Redirect::to(to), "Wrong path"))
    };

    let frag = FragmentPatch {
        fragment_path: path.clone(),
        edition_id: edition.id,
        value: patch.value,
    };
    if let Err(errors) = frag.validate() {
        return Ok(Flash::error(Redirect::to(to), &pack_errs(errors)?));
    }
    if let Err(err) = frag.save(conn) {
        return Ok(Flash::error(
            Redirect::to(to),
            format!(
                "{}",
                json!({
                    "message": format!("{}", err)
                })
            ),
        ));
    }
    Ok(Flash::success(
        Redirect::to(&format!("/ethics/editions/{}/part/{}#{}", &edition_slug, parsed_path.part().unwrap_or(0), path)),
        "",
    ))
}

#[get("/ethics/editions/<edition_slug>/fragments/<path>")]
pub fn ethics_fragment(
    edition_slug: String,
    path: String,
    flash: Option<Flash<()>>,
    conn: DbConn,
) -> Result<Template, Failure> {
    use diesel::*;
    use db::schema::fragments::dsl::*;
    let conn = &*conn.inner().get()?;
    let edition = Edition::by_slug(&edition_slug, &conn)?;
    let frags: Vec<Fragment> = Fragment::belonging_to(&edition)
        .filter(fragment_path.eq(&path))
        .limit(1)
        .load(conn)?;
    let context = json!({
        "fragment": frags.get(0),
        "back": format!("/ethics/editions/{}", edition_slug),
        "url": format!("/ethics/editions/{}/fragments/{}", edition_slug, encode_path(&path.as_bytes())),
        "flash": flash.map(unwrap_flash),
    });
    Ok(Template::render("ethics/fragment", context))
}

#[get("/ethics/editions/<slug>/part/<part>")]
pub fn ethics_part(slug: String, part: u8, conn: DbConn) -> Result<Template, Failure> {
    use diesel::*;
    use db::schema::fragments::dsl::*;
    let conn = &*conn.inner().get()?;
    let editions = Edition::by_slugs(slug.split("+"), &conn)?;
    let frags: Vec<Fragment> = Fragment::belonging_to(&editions)
        .filter(fragment_path.like(format!("pars/{}%", part)))
        .load(conn)?;
    let schemas = ETHICS
        .expand_part(part)
        .into_iter()
        .map(|expanded| {
            editions.iter().map(|edition| {
                let url = format!(
                    "/ethics/editions/{}/fragments/{}",
                    edition.slug,
                    encode_path(expanded.path.as_bytes())
                );
                let frag = frags.iter().find(|f| {
                    f.fragment_path == expanded.path &&
                        f.edition_id == edition.id
                });
                json!({
                    "expanded_node": expanded,
                    "fragment_url": url,
                    "fragment": frag,
                    "rendered": frag.map(|f| ::md_transform::render(&f.value, &slug)),
                })
            }).collect::<Vec<json::Value>>()
        }).collect::<Vec<Vec<json::Value>>>();
    let context = json!({
        "schemas": schemas,
    });
    Ok(Template::render("ethics/part", context))
}

#[get("/ethics/editions/<slug>")]
pub fn ethics_home(slug: String, conn: DbConn) -> Result<Template, Failure> {
    let conn = &*conn.inner().get()?;
    let edition = Edition::by_slug(&slug, &conn)?;
    let context = json!({
        "edition": edition,
        "slug": slug,
    });
    Ok(Template::render("editions/show", context))
}

#[get("/ethics")]
pub fn editions_index(flash: Option<Flash<()>>, conn: DbConn) -> Result<Template, Failure> {
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
        "language_codes": ["fr", "de", "la", "en"],
        "flash": flash
    });
    Template::render("editions/new", &context)
}

#[get("/ethics/editions/<slug>/edit")]
pub fn editions_edit(slug: String, conn: DbConn) -> Result<Template, Failure> {
    let edition = Edition::by_slug(&slug, &*conn.inner().get()?)?;
    let context = json!({ "edition": edition });
    Ok(Template::render("editions/edit", &context))
}

#[post("/ethics/editions/new", data = "<form>")]
pub fn editions_new(form: Form<EditionNew>, conn: DbConn) -> Result<Flash<Redirect>, Failure> {
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
