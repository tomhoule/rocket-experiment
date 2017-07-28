use models;

use diesel::pg::PgConnection;
use rocket::State;
use rocket::request::{Form};
use rocket::response::{NamedFile, Responder};
use rocket_contrib::Json;
// use std::sync::Arc;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

#[post("/api/editions", data="<edition>")]
pub fn editions_create(edition: Json<models::EditionNew>, conn: State<Pool<ConnectionManager<PgConnection>>>) -> Json<String> {
    edition.into_inner().save(&conn.inner().get().unwrap());
    Json("oh".to_string())
}

#[get("/api/editions")]
pub fn editions_index() -> Json<String> {
    unimplemented!();
}

#[get("/api/editions/<id>")]
pub fn edition(id: i32) -> Json<String> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use r2d2;
    use r2d2_diesel;
    use diesel::pg::PgConnection;
    use r2d2_diesel::ConnectionManager;
    use models::EditionNew;
    use json::to_string;
    use rocket;
    use rocket::http::{Accept, ContentType, Header, MediaType, Method, Status};
    use rocket::local::Client;


    pub fn test_post<H, B: AsRef<[u8]>>(body: B, uri: &str, header: H, status: Status)
        where H: Into<Header<'static>>
        {
            let pool_config = r2d2::Config::default();
            let pool_manager = ConnectionManager::<PgConnection>::new(::std::env::var("DATABASE_URL").unwrap());
            let pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Pool::new(pool_config, pool_manager).expect("Failed to create a database connection pool");

            let rocket = rocket::ignite()
                .manage(pool)
                .mount("/", routes![super::editions_create]);

            let client = Client::new(rocket).unwrap();
            let mut response = client.post(uri).header(header).body(body).dispatch();
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
