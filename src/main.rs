//! A tool for making sure your GithubActions configuration files are in in order
use jsonschema_valid::validate;
use lazy_static::lazy_static;
use serde_json::Value;
use std::{error::Error, fs::File, path::PathBuf, process::exit};
use structopt::StructOpt;

lazy_static! {
    static ref WORKFLOW_SCHEMA: Value = serde_json::from_str(include_str!("../data/workflow.json"))
        .expect("invalid workflow schema");
    static ref ACTION_SCHEMA: Value =
        serde_json::from_str(include_str!("../data/action.json")).expect("invalid action schema");
}

#[derive(StructOpt)]
struct Opts {
    path: PathBuf,
}

fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    let Opts { path } = opts;
    println!("validating {}", path.display());
    let result = validate(
        &serde_yaml::from_reader(File::open(path)?)?,
        &WORKFLOW_SCHEMA,
        None,
        true,
    );
    for err in result.get_errors() {
        println!("{}", err);
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
    fn test_valid_01() {
        let result = run(Opts {
            path: "tests/data/workflows/valid_01.yml".into(),
        });
        println!("{:?}", result);
        assert!(result.is_ok())
    }
}
