use models;

use api::error::*;
use diesel::pg::PgConnection;
use rocket::State;
use rocket_contrib::Json;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

type DbConn<'a> = State<'a, Pool<ConnectionManager<PgConnection>>>;

#[post("/api/editions", data = "<edition>")]
pub fn editions_create(
    edition: Json<models::EditionNew>,
    conn: DbConn,
) -> Result<Json<models::Edition>, Error> {
    let edition = edition.into_inner().save(&*conn.inner().get()?)?;
    Ok(Json(edition))
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
    let edition = models::Edition::update(id.parse()?, patch.into_inner(), &*conn.inner().get()?)?;
    Ok(Json(edition))
}

#[cfg(test)]
mod tests {
    use r2d2;
    use diesel::pg::PgConnection;
    use dotenv;
    use r2d2_diesel::ConnectionManager;
    use models::EditionNew;
    use json::to_string;
    use rocket;
    use rocket::http::{ContentType, Header, Status};
    use rocket::local::Client;

    pub fn test_post<H, B: AsRef<[u8]>>(body: B, uri: &str, header: H, status: Status)
    where
        H: Into<Header<'static>>,
    {
        dotenv::dotenv().ok();
        let pool_config = r2d2::Config::default();
        let pool_manager =
            ConnectionManager::<PgConnection>::new(::std::env::var("DATABASE_URL").unwrap());
        let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
            r2d2::Pool::new(pool_config, pool_manager)
                .expect("Failed to create a database connection pool");

        let rocket = rocket::ignite()
            .manage(pool)
            .mount("/", routes![super::editions_create]);

        let client = Client::new(rocket).unwrap();
        let response = client.post(uri).header(header).body(body).dispatch();
        assert_eq!(response.status(), status);
    }

    #[test]
    fn edition_create_happy_path() {
        let payload = EditionNew {
            title: "Collector edition".to_string(),
            editor: "Freddy Mercury".to_string(),
            year: 1977,
            language_code: "en".to_string(),
        };

        let body = to_string(&payload).unwrap();

        test_post(body, "/api/editions", ContentType::JSON, Status::Ok)
    }
}
