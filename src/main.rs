//! A tool for making sure your GithubActions configuration files are in in order
use jsonschema_valid::validate;
use lazy_static::lazy_static;
use serde_json::Value;
use std::{error::Error as StdError, fs, path::PathBuf, process::exit};
use structopt::StructOpt;
mod error;
use error::Error;

lazy_static! {
    static ref WORKFLOW_SCHEMA: Value = serde_json::from_str(include_str!("../data/workflow.json"))
        .expect("invalid workflow schema");
    static ref ACTION_SCHEMA: Value =
        serde_json::from_str(include_str!("../data/action.json")).expect("invalid action schema");
}

#[derive(StructOpt)]
struct Opts {
    path: Vec<PathBuf>,
}

fn run(opts: Opts) -> Result<(), Box<dyn StdError>> {
    let Opts { path } = opts;
    for p in path {
        println!("validating {}", p.display());
        let schema: &Value = if p.file_name().iter().any(|name| "action.yml" == *name) {
            &ACTION_SCHEMA
        } else {
            &WORKFLOW_SCHEMA
        };
        let contents = fs::read_to_string(&p)?;
        let positions = lincolns::from_str(&contents)?;
        let result = validate(&serde_yaml::from_str(&contents)?, schema, None, false);
        if !result.get_errors().is_empty() {
            return Err(Error::Validation(p, positions, result).into());
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run(Opts::from_args()) {
        eprintln!("{}", err);
        exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_schema_is_valid_yaml() {
        let _ = &WORKFLOW_SCHEMA;
    }

    #[test]
    fn action_schema_is_valid_yaml() {
        let _ = &ACTION_SCHEMA;
    }

    #[test]
    fn fails_with_missing_path() {
        assert!(run(Opts {
            path: "tests/data/foobar".into()
        })
        .is_err())
    }

    #[test]
    fn test_workflows_valid_01() {
        let result = run(Opts {
            path: "tests/data/workflows/valid_01.yml".into(),
        });
        assert!(result.is_ok())
    }

    #[test]
    fn test_workflows_invalid_01() {
        let result = run(Opts {
            path: "tests/data/workflows/invalid_01.yml".into(),
        });
        assert!(result.is_err())
    }

    #[test]
    fn test_actions_valid_01() {
        let result = run(Opts {
            path: "tests/data/actions/valid/action.yml".into(),
        });
        assert!(result.is_ok())
    }
}
