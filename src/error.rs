use jsonschema_valid::ValidationErrors;
use std::{error::Error as StdError, fmt};

pub enum Error {
    Validation(ValidationErrors),
}

impl StdError for Error {}

impl fmt::Debug for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Validation(_) => f.write_str("Validation")?,
        }
        Ok(())
    }
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Validation(errors) => {
                for err in errors.get_errors() {
                    write!(f, "{}", err)?;
                }
            }
        }
        Ok(())
    }
}
