use fstlc::{DynamicCombinator, Expr};
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
    if let Err(err) = run(options.subcommand) {
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

    /// The subcommand.
    #[structopt(subcommand)]
    subcommand: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Compiles the given file to Forth code.
    #[structopt(name = "compile")]
    Compile {
        /// The input file.
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        /// The output file.
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        output: Option<PathBuf>,

        /// The prefix used for generated definitions.
        #[structopt(short = "p", long = "prefix")]
        prefix: Option<String>,
    },

    /// Compiles the given file to combinators.
    #[structopt(name = "compile-to-combinators")]
    CompileToCombinators {
        /// The input file.
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },

    /// Evaluates the given file as combinators.
    #[structopt(name = "eval-combinators")]
    EvalCombinators {
        /// The input file.
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

fn run(command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Compile {
            input,
            output,
            prefix,
        } => {
            let src = read_to_string(&input)?;
            let expr: Expr = src.parse()?;
            let ty = expr.tyck()?;
            let prefix = prefix
                .or(input.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .ok_or("Cannot determine prefix")?;

            let mut forth = include_str!("prelude.f").to_string();
            forth.push('\n');
            forth += &format!("\\ expr = {}\n\\ type = {}\n", expr, ty);
            for decl in expr.compile(&prefix)? {
                forth += &itertools::join(decl, " ");
                forth.push('\n');
            }

            if let Some(path) = output {
                write(path, forth.as_bytes())?;
            } else {
                stdout().write_all(forth.as_bytes())?;
            }
            Ok(())
        }

        Command::CompileToCombinators { input } => {
            let src = read_to_string(&input)?;
            let expr: Expr = src.parse()?;
            expr.tyck()?;
            println!("{}", expr.to_combinators()?);
            Ok(())
        }

        Command::EvalCombinators { input } => {
            let src = read_to_string(&input)?;
            let expr: Expr = src.parse()?;
            expr.tyck()?;
            let combinator = expr.to_combinators()?;
            let mut last = DynamicCombinator::Apply(
                Box::new(DynamicCombinator::from(combinator)),
                Box::new(DynamicCombinator::Name("end-of-env".into())),
            );
            loop {
                println!("{}", last);
                let next = last.clone().eval_step();
                if next == last {
                    break Ok(());
                }
                last = next;
            }
        }
    }
}
