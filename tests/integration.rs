#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate diesel;
extern crate dotenv;
extern crate fantoccini;
extern crate futures_await as futures;
extern crate tokio_core;

use diesel::*;
use futures::prelude::*;
use fantoccini::Client;
use std::rc::Rc;
use std::env;

fn wait_ms(ms: u64) {
    ::std::thread::sleep(::std::time::Duration::from_millis(ms));
}

#[test]
fn integration() {
    dotenv::dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("database url");
    env::set_var("DATABASE_URL", &database_url);

    let conn = PgConnection::establish(&database_url).expect("Database is up");

    loop {
        if diesel::migrations::revert_latest_migration(&conn).is_err() {
            break
        }
    }
    diesel::migrations::run_pending_migrations(&conn).unwrap();

    let mut driver = ::std::process::Command::new("geckodriver")
        .spawn()
        .expect("Could not start geckodriver");

    ::std::thread::sleep(::std::time::Duration::from_millis(100));

    let mut child = ::std::process::Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("Could not start server");

    ::std::thread::sleep(::std::time::Duration::from_millis(300));

    let result = ::std::panic::catch_unwind(|| {
        let mut core = tokio_core::reactor::Core::new().expect("started tokio");
        let (c, fin) = Client::new("http://localhost:4444", &core.handle());
        let client = core.run(c).expect("created client");
        core.run(tests(Rc::new(client))).expect("tests failed");
        // and wait for cleanup to finish
        core.run(fin).ok();
    });

    child.kill().ok();
    driver.kill().ok();

    result.expect("tests failed");
}

#[async]
fn tests(client: Rc<fantoccini::Client>) -> Result<(), fantoccini::error::CmdError> {
    let c = client.clone();
    let app_url = "http://127.0.0.1:8000";

    await!(c.clone().goto(app_url))?;
    let url = await!(c.current_url())?;
    assert_eq!(url.as_ref(), &format!("{}/", app_url));
    let link = await!(c.by_selector("a[data-book=\"ethics\"]"))?;
    await!(link.click())?;
    let url = await!(c.current_url())?;
    assert_eq!(url.as_ref(), &format!("{}/ethics", app_url));
    let link = await!(c.by_link_text("Make a new one"))?;
    await!(link.click())?;
    let url = await!(c.current_url())?;
    assert_eq!(url.as_ref(), &format!("{}/ethics/editions/create", app_url));

    let form = await!(c.form("form"))?;
    await!(form.set_by_name("title", "test edition fren"))?;
    await!(form.submit())?;
    assert_eq!(url.as_ref(), &format!("{}/ethics/editions/create", app_url));
    let form = await!(c.form("form"))?;
    await!(form.set_by_name("title", "test edition fren"))?;
    await!(form.set_by_name("editor", "testditor"))?;
    await!(form.set_by_name("slug", "test_ed"))?;
    await!(form.set_by_name("year", "2010"))?;
    let option = await!(c.by_selector("option[value=de]"))?;
    await!(option.click())?;
    await!(form.submit())?;
    assert_eq!(await!(c.current_url())?.as_ref(), &format!("{}/ethics", app_url));

    Ok(())
}