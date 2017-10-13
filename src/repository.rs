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

mod db;
mod models;
mod rpc;
mod schemas;

use futures::Future;
use schemas::ethica::ETHICA;

#[derive(Clone)]
struct Server;

impl rpc::repository_grpc::EthicsRepository for Server {
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

    let env = grpcio::Environment::new(1);
    let service = rpc::repository_grpc::create_ethics_repository(Server);
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
