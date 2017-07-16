use serde::de::value::Error as SerdeError;

error_chain! {
    types {
        ValidationError, ValidationErrorKind, ResultExt;
    }

    errors {
        Empty {
            description("Non-empty field validation")
            display("Field must not be empty")
        }
    }

    links {}

    foreign_links {
        Serde(SerdeError);
    }
}
