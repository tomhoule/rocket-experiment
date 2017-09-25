use rocket::FromRequest;

trait FlashMessageClass {
    const KEY: &'static str;

    fn extract_flash(flash: &FlashMessage) -> Option<&str>
    where MessageClass: FlashMessageClass
    {
        if flash.name() == Self::KEY {
            Some(flash.msg().to_string())
        } else {
            None
        }
    }
}

impl<MessageClass> FromRequest for MessageClass
where MessageClass: FlashMessageClass
{
    fn from_request(request: Request) -> Outcome<MessageClass> {
        let flash = FlashMessage::from_request(request);
        MessageClass::extract_flash(flash);
}

pub struct ErrorFlash(pub ::json::Value);

impl FromRequest for ErrorFlash {
    const KEY: "error";
}

pub struct SuccessFlash(pub ::json::Value);

impl FromRequest for SuccessFlash {
    const KEY: "error";
}
