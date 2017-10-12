use api::error::*;
use rocket::Outcome;
use rocket::request::{FlashMessage, FromRequest, Request, Outcome as ROutcome};

trait FlashMessageClass {
    const KEY: &'static str;

    fn extract_flash<MessageClass>(flash: &FlashMessage) -> Option<&str>
    where MessageClass: FlashMessageClass
    {
        if flash.name() == Self::KEY {
            Some(flash.msg())
        } else {
            None
        }
    }
}

impl<'a, 'b, MessageClass> FromRequest<'a, 'b> for Flash<MessageClass>
where MessageClass: FlashMessageClass
{

    type Error = Error;

    fn from_request(request: &Request) -> ROutcome<Flash<MessageClass>, Error> {
        let flash = match FlashMessage::from_request(request) {
            Outcome::Success(flash) => flash,
            Outcome::Failure(err) => return Outcome::Failure(err.into()),
            Outcome::Forward(()) => return Outcome::Forward(()),
        };

        match <MessageClass as FlashMessageClass>::extract_flash(&flash) {
            Some(msg) => Outcome::Success(Flash(msg)),
            None => Outcome::Forward(())
        }
    }
}

struct Flash<T>(pub T);

struct InnerSuccessFlash<T>(pub T);
pub type SuccessFlash<T> = Flash<InnerSuccessFlash<T>>

impl FlashMessageClass for SuccessFlash {
    const KEY: &'static str = "success";
}

struct InnerErrorFlash<T>(pub T);
pub type ErrorFlash<T> = Flash<InnerErrorFlash<T>>

impl FlashMessageClass for ErrorFlash {
    const KEY: &'static str = "error";
}
