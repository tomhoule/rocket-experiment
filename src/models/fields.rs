use rocket::request::*;
use rocket::http::RawStr;
use models::validation::*;
use serde::de::*;

#[derive(Debug)]
pub struct NonEmptyString(String);

impl<'d> Deserialize<'d> for NonEmptyString {
    fn deserialize<D: Deserializer<'d>>(de: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(de)?;

        if raw.is_empty() {
            return Err(D::Error::invalid_length(0, &"non-empty string"))
        }

        Ok(NonEmptyString(raw))
    }
}

impl<'v> FromFormValue<'v> for NonEmptyString {
    type Error = ValidationError;

    fn from_form_value(form_value: &'v RawStr) -> Result<NonEmptyString, Self::Error> {
        if form_value.is_empty() {
            Err(ValidationErrorKind::Empty.into())
        } else {
            Ok(NonEmptyString(form_value.to_string()))
        }
    }
}
