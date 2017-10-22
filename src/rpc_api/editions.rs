use uuid;
use models;
use error::Error;
use super::repository::Repository;
use diesel::pg::PgConnection;
use rpc::repository::*;

pub fn get_editions(
    _ctx: &Repository,
    _req: GetEditionsParams,
    conn: &PgConnection,
) -> Result<Editions, Error> {
    use protobuf::RepeatedField;
    use std::iter::*;

    let mut response = Editions::new();
    let editions = models::Edition::all(&conn).map_err(Error::from)?;
    let transformed = editions.into_iter().map(|ed| ed.into_proto());
    response.set_data(RepeatedField::from_iter(transformed));
    Ok(response)
}

pub fn edit_edition(
    _ctx: &Repository,
    req: Edition,
    conn: &PgConnection,
) -> Result<Edition, Error> {
    let id: uuid::Uuid = req.id.parse()?;
    Ok(
        models::EditionPatch::from_proto(req)
            .save(id, conn)?
            .into_proto(),
    )
}

pub fn create_edition(
    _ctx: &Repository,
    req: Edition,
    conn: &PgConnection,
) -> Result<Edition, Error> {
    Ok(
        models::EditionNew::from_proto(req)?
            .save(conn)?
            .into_proto(),
    )
}

pub fn delete_edition(
    _ctx: &Repository,
    req: Edition,
    conn: &PgConnection,
) -> Result<Empty, Error> {
    models::Edition::delete(req.id.parse()?, conn)?;
    Ok(Empty::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv;
    use error::*;

    #[test]
    fn create_edition_validates() {
        dotenv::dotenv().ok();
        let repo = Repository::new();
        let conn = repo.pool.get().unwrap();
        let req = Edition::new();
        let res = create_edition(&repo, req, &conn);
        match res {
            Err(Error(ErrorKind::Validation(_), _)) => (),
            other => panic!("Unexpected: {:?}", other),
        }
    }
}
