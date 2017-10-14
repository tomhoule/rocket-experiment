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
use std::path::{Path, PathBuf};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod api;
mod db;
mod models;
mod rpc;
mod schemas;

use futures::Future;
use schemas::ethica::ETHICA;

#[derive(Clone)]
struct Repository {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}

impl Repository {
    fn handle(&self, err: api::Error) {
        panic!("{}", err);
    }
}

impl rpc::repository_grpc::EthicsRepository for Repository {
    fn get_schema(
        &self,
        ctx: ::grpcio::RpcContext,
        req: rpc::repository::GetSchemaParams,
        sink: ::grpcio::UnarySink<rpc::repository::EthicsSchema>
    ) {
        use protobuf::RepeatedField;

        let mut schema = rpc::repository::EthicsSchema::new();
        let parts = ETHICA.0.iter().map(|node| node.to_protobuf()).collect();
        schema.set_parts(RepeatedField::from_vec(parts));
        let f = sink.success(schema)
            .map_err(|err| panic!("Errored with {}", err));
        ctx.spawn(f)
    }

    fn get_editions(
        &self,
        ctx: ::grpcio::RpcContext,
        req: rpc::repository::GetEditionsParams,
        sink: ::grpcio::UnarySink<rpc::repository::Editions>
    ) {
        use protobuf::RepeatedField;
        use ::std::iter::*;

        let mut response = rpc::repository::Editions::new();
        let editions = self.pool.get()
            .map_err(api::Error::from)
            .and_then(|conn| models::Edition::all(&conn).map_err(api::Error::from));
        match editions {
            Ok(editions) => {
                let transformed = editions.iter().map(|ed| rpc::repository::Edition::new());
                response.set_data(RepeatedField::from_iter(transformed));
                let f = sink.success(response)
                    .map_err(|err| panic!("Errored with {}", err));
                ctx.spawn(f);
            },
            Err(err) => self.handle(err)
        }
    }

    fn create_edition(
        &self,
        ctx: ::grpcio::RpcContext,
        req: rpc::repository::Edition,
        sink: ::grpcio::UnarySink<rpc::repository::Edition>
    ) {
        let edition = models::EditionNew::from_protobuf(req);
        let saved = self.pool.get()
            .map_err(api::Error::from)
            .and_then(|conn| edition.save(&conn).map_err(api::Error::from));
        match saved {
            Ok(edition) =>
                ctx.spawn(sink.success(edition.to_protobuf()).map_err(|err| panic!("err {}", err))),
            Err(err) => self.handle(err),
        }
    }

    fn patch_edition(
        &self,
        ctx: ::grpcio::RpcContext,
        mut req: rpc::repository::EditionPatch,
        sink: ::grpcio::UnarySink<rpc::repository::Edition>
    ) {
        let uuid = req.take_id();
        let patch = models::EditionPatch::from_proto(req);
        let saved = self.pool.get()
            .map_err(api::Error::from)
            .and_then(|conn| patch.save(uuid, &conn).map_err(api::Error::from));
        match saved {
            Ok(edition) =>
                ctx.spawn(sink.success(edition.to_protobuf()).map_err(|err| panic!("GRPC error"))),
            Err(err) => self.handle(err),
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    let pool_config = r2d2::Config::default();
    let database_url = ::std::env::var("DATABASE_URL").unwrap();
    let pool_manager =
        ConnectionManager::<PgConnection>::new(database_url);
    println!("Connecting to database");
    let pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");

    let repo = Repository { pool };
    let env = grpcio::Environment::new(1);
    let service = rpc::repository_grpc::create_ethics_repository(repo);
    let port = 9090;
    let mut server = grpcio::ServerBuilder::new(::std::sync::Arc::new(env))
        .bind("localhost", port)
        .register_service(service)
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
