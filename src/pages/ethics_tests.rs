use dotenv;
use models::edition::*;
use diesel::prelude::*;
use rocket::local::*;
use rocket::http;
use std::time::{SystemTime, Duration};

fn conn() -> PgConnection {
    dotenv::dotenv().ok();
    let db_str = ::std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL env var is present");
    PgConnection::establish(&db_str)
        .expect("can connect to database")
}

#[test]
fn test_ethics_part() {
    let conn = conn();
    conn.begin_test_transaction().unwrap();
    let edition = EditionNew {
        title: "TestEdition".to_string(),
        editor: "TestEditor".to_string(),
        year: 1666,
        language_code: "de".to_string(),
        slug: "tested".to_string(),
    };
    edition.save(&conn).unwrap();
    let app = ::start();
    let client = Client::new(app).unwrap();

    let start = SystemTime::now();
    let mut res = client.get("/ethics/editions/tested/part/1").dispatch();
    let body = res.body_string().unwrap();
    assert_eq!(res.status(), http::Status::Ok);
    assert!(body.len() > 1000);
    let elapsed = SystemTime::now().duration_since(start).unwrap();

    assert!(elapsed < Duration::from_millis(40), "Elapsed: {:?}", elapsed);
}
