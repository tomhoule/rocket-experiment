use r2d2;
use error::Error;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

#[derive(Clone)]
pub struct Repository {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Repository {
    pub fn new() -> Self {
        let pool_config = r2d2::Config::default();
        let database_url = ::std::env::var("DATABASE_URL").unwrap();
        let pool_manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(pool_config, pool_manager)
            .expect("Failed to create a database connection pool");
        Repository { pool }
    }

    pub fn with_connection<Req, Res>(
        &self,
        req: Req,
        inner: &Fn(&Self, Req, &PgConnection) -> Result<Res, Error>,
    ) -> Result<Res, Error> {
        self.pool
            .get()
            .map_err(Error::from)
            .and_then(|conn| inner(self, req, &conn))
    }
}
