extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate purescript_waterslide;
#[macro_use]
extern crate purescript_waterslide_derive;
extern crate r2d2;
extern crate r2d2_diesel;
// extern crate rocket;
// extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate uuid;
#[macro_use]
extern crate validator_derive;
extern crate validator;

// use rocket::response::NamedFile;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod api;
mod db;
mod models;
mod rpc;
mod schemas;

use futures::Future;
use schemas::ethica::ETHICA;

fn bail(err: grpcio::Error) {
    panic!("{}", err)
}

#[derive(Clone)]
struct Repository {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl Repository {
    fn handle(&self, err: api::Error) {
        panic!("{}", err);
    }

    fn with_connection<Req, Res>(
        &self,
        ctx: ::grpcio::RpcContext,
        req: Req,
        sink: ::grpcio::UnarySink<Res>,
        inner: &Fn(&Self, Req, &PgConnection) -> Result<Res, api::Error>
    ) {
        self.pool.get()
            .map_err(api::Error::from)
            .and_then(|conn| inner(self, req, &conn))
            .and_then(|res| {
                ctx.spawn(sink.success(res).map_err(bail));
                Ok(())
            })
            .map_err(|err| self.handle(err))
            .ok();
    }
}


fn get_editions(
    _ctx: &Repository,
    _req: rpc::repository::GetEditionsParams,
    conn: &PgConnection
) -> Result<rpc::repository::Editions, api::Error> {
    use protobuf::RepeatedField;
    use ::std::iter::*;

    let mut response = rpc::repository::Editions::new();
    let editions = models::Edition::all(&conn).map_err(api::Error::from)?;
    let transformed = editions.into_iter().map(|ed| ed.to_proto());
    response.set_data(RepeatedField::from_iter(transformed));
    Ok(response)
}

impl rpc::repository_grpc::EthicsRepository for Repository {
    fn get_schema(
        &self,
        ctx: ::grpcio::RpcContext,
        _req: rpc::repository::GetSchemaParams,
        sink: ::grpcio::UnarySink<rpc::repository::EthicsSchema>
    ) {
        use protobuf::RepeatedField;
        use ::std::iter::*;

        let mut schema = rpc::repository::EthicsSchema::new();
        let parts = ETHICA.0.iter().map(|node| node.to_protobuf());
        schema.set_parts(RepeatedField::from_iter(parts));
        ctx.spawn(sink.success(schema).map_err(bail));
    }

    fn get_editions(
        &self,
        ctx: ::grpcio::RpcContext,
        req: rpc::repository::GetEditionsParams,
        sink: ::grpcio::UnarySink<rpc::repository::Editions>
    ) {
        self.with_connection(ctx, req, sink, &get_editions);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let pool_config = r2d2::Config::default();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    let pool_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");

    let repo = Repository { pool };
    let env = grpcio::Environment::new(4);
    let port = 9090;
    let mut server = grpcio::ServerBuilder::new(::std::sync::Arc::new(env))
        .bind("localhost", port)
        .register_service(rpc::repository_grpc::create_ethics_repository(repo))
        .build()
        .expect("Server misconfiguration");

    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("Listening on {}:{}", host, port);
    }

    ::std::thread::sleep(::std::time::Duration::from_millis(999999999));

    println!("Shutting down");
    let _ = server.shutdown().wait().unwrap();
}
