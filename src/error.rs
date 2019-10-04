use jsonschema_valid::ValidationErrors;
use lazy_static::lazy_static;
use lincolns::Positions;
use regex::Regex;
use std::{error::Error as StdError, fmt, path::PathBuf};

pub enum Error {
    Validation(PathBuf, Positions, ValidationErrors),
}

impl StdError for Error {}

impl fmt::Debug for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Error::Validation(_, _, _) => f.write_str("Validation(...)")?,
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
            Error::Validation(path, pos, errors) => {
                for err in errors.get_errors() {
                    // work around until https://github.com/mdboom/jsonschema-valid/issues/2
                    lazy_static! {
                        static ref RE: Regex = Regex::new(r#"At (.+) with schema at (.+): (.+)"#)
                            .expect("invalid regex");
                    };
                    let err_str = err.to_string();
                    let caps = RE
                        .captures(err_str.as_str())
                        .unwrap_or_else(|| panic!("{} didn't match format", err_str));
                    let field = caps.get(1).map(|c| c.as_str()).unwrap_or_default();
                    let msg = caps.get(3).map(|c| c.as_str()).unwrap_or_default();
                    let lincolns::Position { line, col } = pos
                        .get(format!("/{}", field))
                        .unwrap_or_else(|| &lincolns::Position { line: 1, col: 1 });
                    if let Ok(src) = source_error::from_file(
                        msg,
                        &path,
                        source_error::Position::new(*line, *col),
                    ) {
                        write!(f, "{}", src)?;
                    }
                }
            }
        }
        Ok(())
    }
}
