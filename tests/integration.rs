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

fn wait_ms_fut(ms: u64, handle: &tokio_core::reactor::Handle) -> tokio_core::reactor::Timeout {
    tokio_core::reactor::Timeout::new(::std::time::Duration::from_millis(ms), &handle).unwrap()
}

const APP_URL: &'static str = "http://127.0.0.1:8000";

struct TestContext {
    client: Rc<fantoccini::Client>,
    handle: tokio_core::reactor::Handle,
}

#[test]
fn integration() {
    dotenv::dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("database url");
    env::set_var("DATABASE_URL", &database_url);

    let conn = PgConnection::establish(&database_url).expect("Database is up");

    loop {
        if diesel::migrations::revert_latest_migration(&conn).is_err() {
            break;
        }
    }
    diesel::migrations::run_pending_migrations(&conn).unwrap();

    let mut driver = ::std::process::Command::new("geckodriver")
        .spawn()
        .expect("Could not start geckodriver");

    let mut child = ::std::process::Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("Could not start server");

    wait_ms(60);

    let result = ::std::panic::catch_unwind(|| {
        let mut core = tokio_core::reactor::Core::new().expect("started tokio");
        let (c, fin) = Client::new("http://localhost:4444", &core.handle());
        let client = core.run(c).expect("created client");
        let ctx = TestContext {
            client: Rc::new(client),
            handle: core.handle(),
        };
        core.run(tests(ctx)).expect("tests failed");
        // and wait for cleanup to finish
        core.run(fin).ok();
    });

    child.kill().ok();
    driver.kill().ok();

    result.expect("tests failed");
}

#[async]
fn tests(c: TestContext) -> Result<(), fantoccini::error::CmdError> {
    await!(create_edition(c.client.clone()))?;
    await!(edit_fragment(c.client.clone(), c.handle.clone()))?;
    Ok(())
}

#[async]
fn create_edition(c: Rc<fantoccini::Client>) -> Result<(), fantoccini::error::CmdError> {
    await!(c.goto(APP_URL))?;
    assert_eq!(await!(c.current_url())?.as_ref(), &format!("{}/", APP_URL));
    let link = await!(c.by_selector("a[data-book=\"ethics\"]"))?;
    await!(link.click())?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!("{}/ethics", APP_URL)
    );
    let link = await!(c.by_link_text("Make a new one"))?;
    await!(link.click())?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!("{}/ethics/editions/create", APP_URL)
    );

    let form = await!(c.form("form"))?;
    await!(form.set_by_name("title", "test edition fren"))?;
    await!(form.submit())?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!("{}/ethics/editions/create", APP_URL)
    );
    let form = await!(c.form("form"))?;
    await!(form.set_by_name("title", "test edition fren"))?;
    await!(form.set_by_name("editor", "testditor"))?;
    await!(form.set_by_name("slug", "test_ed"))?;
    await!(form.set_by_name("year", "2010"))?;
    await!(await!(c.by_selector("option[value=de]"))?.click())?;
    await!(form.submit())?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!("{}/ethics", APP_URL)
    );

    Ok(())
}

#[async]
fn edit_fragment(c: Rc<fantoccini::Client>, h: tokio_core::reactor::Handle) -> Result<(), fantoccini::error::CmdError> {
    await!(c.goto(&format!("{}/ethics/editions/test_ed", APP_URL)))?;
    let link = await!(c.by_link_text("Part 2"))?;
    await!(link.click())?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!("{}/ethics/editions/test_ed/part/2", APP_URL)
    );
    await!(wait_ms_fut(800, &h));
    await!(await!(c.by_selector("i.fa-link"))?.click())?;

    let form = await!(c.form("form"))?;
    // await!(form.set_by_name("value", "meow, said the cat"))?;
    assert_eq!(
        await!(c.current_url())?.as_ref(),
        &format!(
            "{}/ethics/editions/test_ed/fragments/pars%2F2",
            APP_URL
        )
    );
    await!(form.submit())?;
    // assert_eq!(
    //     await!(c.current_url())?.as_ref(),
    //     &format!("{}/ethics", APP_URL)
    // );
    Ok(())
}
