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

    /// The prefix used for generated definitions.
    #[structopt(short = "p", long = "prefix")]
    prefix: Option<String>,
}

fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let src = read_to_string(&options.input)?;
    let expr: Expr = src.parse()?;
    let ty = expr.tyck()?;
    let prefix = options
        .prefix
        .or(options
            .input
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned()))
        .ok_or("Cannot determine prefix")?;

    let mut forth = format!("\\ expr = {}\n\\ type = {}\n", expr, ty);
    forth.extend(include_str!("prelude.f").chars());
    for decl in expr.compile(&prefix)? {
        forth += &itertools::join(decl, " ");
        forth.push('\n');
    }
    println!("{}", forth);

    write(options.output, forth.as_bytes())?;
    Ok(())
}
