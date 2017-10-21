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
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate uuid;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

mod db;
mod error;
mod models;
mod rpc;
mod schemas;

use error::Error;
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
    fn with_connection<Req, Res>(
        &self,
        req: Req,
        inner: &Fn(&Self, Req, &PgConnection) -> Result<Res, Error>
    ) -> Result<Res, Error> {
        self.pool.get()
            .map_err(Error::from)
            .and_then(|conn| inner(self, req, &conn))
    }
}


macro_rules! handler {
    ($name:ident, $req:path, $res:path, $inner:expr) => {
        fn $name(
            &self,
            ctx: ::grpcio::RpcContext,
            req: $req,
            sink: ::grpcio::UnarySink<$res>
        ) {
            match $inner(&self, req) {
                Ok(res) => ctx.spawn(sink.success(res).map_err(bail)),
                Err(err) => ctx.spawn(sink.fail(err.into_grpc_status()).map_err(bail)),
            }
        }
    }
}


fn get_editions(
    _ctx: &Repository,
    _req: rpc::repository::GetEditionsParams,
    conn: &PgConnection
) -> Result<rpc::repository::Editions, Error> {
    use protobuf::RepeatedField;
    use ::std::iter::*;

    let mut response = rpc::repository::Editions::new();
    let editions = models::Edition::all(&conn).map_err(Error::from)?;
    let transformed = editions.into_iter().map(|ed| ed.to_proto());
    response.set_data(RepeatedField::from_iter(transformed));
    Ok(response)
}

fn get_fragments(
    _ctx: &Repository,
    req: rpc::repository::GetFragmentsParams,
    conn: &PgConnection
) -> Result<rpc::repository::EthicsFragments, Error> {
    use rpc::repository::*;
    let mut response = EthicsFragments::new();
    let fragments = models::Fragment::for_edition(&req.edition_id, conn)?;
    {
        let mut map = response.mut_fragments();
        for fragment in fragments.into_iter() {
            map.insert(fragment.fragment_path.clone(), fragment.into_proto());
        }
    }
    Ok(response)
}

fn get_schema(
    ctx: &Repository,
    req: rpc::repository::GetSchemaParams
) -> Result<rpc::repository::EthicsSchema, Error> {
    use protobuf::RepeatedField;
    use ::std::iter::*;

    let mut schema = rpc::repository::EthicsSchema::new();
    let parts = ETHICA.0.iter().map(|node| node.to_protobuf());
    schema.set_parts(RepeatedField::from_iter(parts));
    Ok(schema)
}

fn edit_fragment(
    ctx: &Repository,
    req: rpc::repository::EthicsFragment,
    conn: &PgConnection,
) -> Result<rpc::repository::EthicsFragment, Error> {
    Ok(models::FragmentPatch::from_proto(req)?.save(conn)?.into_proto())
}

fn dead_end<T, U>(ctx: &Repository, _req: T) -> Result<U, Error> {
    unimplemented!()
}

impl rpc::repository_grpc::EthicsRepository for Repository {
    handler! {
        get_schema,
        rpc::repository::GetSchemaParams,
        rpc::repository::EthicsSchema,
        get_schema
    }

    handler! {
        get_fragments,
        rpc::repository::GetFragmentsParams,
        rpc::repository::EthicsFragments,
        |s: &Repository, req| { s.with_connection(req, &get_fragments) }
    }

    handler! {
        edit_fragment,
        rpc::repository::EthicsFragment,
        rpc::repository::EthicsFragment,
        |s: &Repository, req| { s.with_connection(req, &edit_fragment) }
    }

    handler! {
        get_editions,
        rpc::repository::GetEditionsParams,
        rpc::repository::Editions,
        |itself: &Repository, req| { itself.with_connection(req, &get_editions) }
    }

    handler! {
        create_edition,
        rpc::repository::Edition,
        rpc::repository::Edition,
        dead_end
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
