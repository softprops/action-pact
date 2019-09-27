use std::{error::Error, path::PathBuf, process::exit};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    path: PathBuf,
}

fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    let Opts { path } = opts;
    println!("{}", path.display());
    Ok(())
}

fn main() {
    if let Err(err) = run(Opts::from_args()) {
        eprintln!("{}", err);
        exit(1)
    }
}
