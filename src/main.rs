use fstlc::Expr;
use std::{
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
    process::exit,
};
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    if let Err(err) = run(options) {
        eprintln!("{}", err);
        exit(1);
    }
}

#[derive(StructOpt)]
struct Options {
    /// The input file.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// The top-level name.
    #[structopt(short = "n", long = "name", default_value = "main")]
    name: String,
}

fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let src = read_to_string(options.input)?;
    let expr: Expr = src.parse()?;
    let ty = expr.tyck()?;
    let chunks = expr.compile()?;

    let mut forth = format!("\\ expr = {}\n\\ type = {}\n", expr, ty);
    forth.extend(include_str!("prelude.f").chars());
    forth.extend(itertools::join(chunks, " ").chars());
    println!("{}", forth);

    write(options.output, forth.as_bytes())?;
    Ok(())
}
