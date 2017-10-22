use protobuf::Message;
use error::Error;
use super::repository::Repository;

pub trait Summonable: Sized {
    fn summon<T: Message>(req: T, context: &Repository) -> Result<Self, Error>;
}
