use fstlc::Expr;
use std::{
    error::Error,
    fs::{read_to_string, write},
    io::{stdout, Write},
    path::PathBuf,
    process::exit,
};
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .init()
        .unwrap();
    if let Err(err) = run(options) {
        eprintln!("{}", err);
        exit(1);
    }
}

#[derive(StructOpt)]
struct Options {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// The input file.
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

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

    let mut forth = include_str!("prelude.f").to_string();
    forth.push('\n');
    forth += &format!("\\ expr = {}\n\\ type = {}\n", expr, ty);
    for decl in expr.compile(&prefix)? {
        forth += &itertools::join(decl, " ");
        forth.push('\n');
    }

    if let Some(path) = options.output {
        write(path, forth.as_bytes())?;
    } else {
        stdout().write_all(forth.as_bytes())?;
    }
    Ok(())
}
