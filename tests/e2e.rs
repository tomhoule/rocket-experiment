extern crate dotenv;
extern crate locutions;
extern crate diesel;
extern crate reqwest;

use diesel::connection::Connection;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use reqwest::StatusCode;

fn setup() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).expect("Database is up")
}

#[test]
fn e2e() {
    use locutions::models::edition::*;
    use locutions::db::schema::editions::dsl::*;

    let conn = setup();
    diesel::migrations::run_pending_migrations(&conn).unwrap();
    let client = reqwest::Client::new();
    let api_url = "http://localhost:8000";

    let _handle = ::std::thread::spawn(|| {
        locutions::start();
    });

    diesel::delete(editions).execute(&conn).unwrap();
    ::std::thread::sleep(::std::time::Duration::from_millis(100));

    let payload = EditionNew {
        title: "Collector edition".to_string(),
        editor: "Freddy Mercury".to_string(),
        slug: "collector".to_string(),
        year: 1977,
        language_code: "en".to_string(),
    };
    let res = client.post(&format!("{}{}", api_url, "/api/editions"))
        .json(&payload)
        .send()
        .expect("api is reachable");

    assert_eq!(res.status(), StatusCode::Created);
}
