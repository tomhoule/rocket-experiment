use r2d2;
use error::Error;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

#[derive(Clone)]
pub struct Repository {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Repository {
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
