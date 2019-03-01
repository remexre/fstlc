use crate::Expr;

/// A "chunk" of Forth code.
///
/// For the purposes of this compiler, a chunk is a word or set of words that execute "as one."
/// For example, `3 PICK` is one chunk, but 2 words.
#[derive(Debug, Display)]
pub enum Chunk {
    /// Prints a number. `( x -- x )`
    #[display(fmt = "dup .")]
    Print,

    /// Pushes a word's execution token to the stack. `( -- xt )`
    #[display(fmt = "[ ' {} ] literal", _0)]
    Quoted(&'static str),

    /// Executes a word on the stack. `( i * x xt -- j * x )`
    #[display(fmt = "execute")]
    Execute,

    /// Defines a function.
    #[display(fmt = ": {}", _0)]
    Def(String),

    /// Ends the definition of a function.
    #[display(fmt = ";")]
    EndDef,

    /// Pushes a literal to the stack. `( -- x )`
    Lit(u32),

    /// Copies an item from lower on the stack. `( x_u ... x_0 u -- x_u ... x_0 x_u )`
    #[display(fmt = "{} pick", _0)]
    Pick(usize),

    /// Drops some number of items from the stack. `( x_u ... x_0 u -- )`
    #[display(fmt = "{} discard", _0)]
    Discard(usize),

    /// Copies some number of words from the heap to the stack. `( addr u -- x_u ... x_0 )`
    #[display(fmt = "{} fstlc-read-block", _0)]
    ReadBlock(usize),

    /// Writes some number of words from the stack to the heap. `( x_u ... x_0 u -- addr )`
    #[display(fmt = "{} fstlc-write-block", _0)]
    WriteBlock(usize),
}

pub fn compile(
    expr: &NamelessExpr,
    depth: &mut usize,
    hoist: &mut Vec<(String, Vec<Chunk>, usize)>,
) -> Vec<Chunk> {
    match expr {
        NamelessExpr::Plus => unimplemented!(),
        NamelessExpr::Minus => unimplemented!(),
        NamelessExpr::Times => unimplemented!(),
        NamelessExpr::Divide => unimplemented!(),
        NamelessExpr::Mod => unimplemented!(),
        NamelessExpr::Print => unimplemented!(),
        NamelessExpr::App(l, r) => {
            let mut r = compile(r, depth, hoist);
            r.extend(compile_call(l, depth, hoist));
            r
        }
        NamelessExpr::Lam(e) => unimplemented!(),
        NamelessExpr::Lit(n) => {
            *depth += 1;
            vec![Chunk::Lit(*n)]
        }
        NamelessExpr::Var(n) => {
            *depth += 1;
            vec![Chunk::Pick(*n)]
        }
    }
}

fn compile_call(
    expr: &NamelessExpr,
    depth: &mut usize,
    hoist: &mut Vec<(String, Vec<Chunk>, usize)>,
) -> Vec<Chunk> {
    match expr {
        NamelessExpr::Plus => {
            *depth -= 1;
            vec![Chunk::Quoted("+")]
        }
        NamelessExpr::Minus => {
            *depth -= 1;
            vec![Chunk::Quoted("-")]
        }
        NamelessExpr::Times => {
            *depth -= 1;
            vec![Chunk::Quoted("*")]
        }
        NamelessExpr::Divide => {
            *depth -= 1;
            vec![Chunk::Quoted("/")]
        }
        NamelessExpr::Mod => {
            *depth -= 1;
            vec![Chunk::Quoted("mod")]
        }
        NamelessExpr::Print => vec![Chunk::Print],
        NamelessExpr::App(l, r) => {
            let mut r = compile(r, depth, hoist);
            r.extend(compile_call(l, depth, hoist));
            r
        }
        NamelessExpr::Lam(e) => unimplemented!(),
        NamelessExpr::Lit(n) => panic!("{} is not callable!", n),
        NamelessExpr::Var(n) => unimplemented!(),
    }
}

/// A nameless expression.
#[derive(Debug)]
pub enum NamelessExpr {
    /// The function that adds two numbers.
    Plus,

    /// The function that subtracts two numbers.
    Minus,

    /// The function that multiplies two numbers.
    Times,

    /// The function that divides two numbers.
    Divide,

    /// The function that computes the modulus between two numbers.
    Mod,

    /// The function that prints a number.
    Print,

    /// Function application.
    App(Box<NamelessExpr>, Box<NamelessExpr>),

    /// Function abstraction.
    Lam(Box<NamelessExpr>),

    /// A literal value.
    Lit(u32),

    /// A variable reference.
    Var(usize),
}

pub fn remove_names<'e>(scope: &mut Vec<&'e str>, expr: &'e Expr) -> Result<NamelessExpr, String> {
    match expr {
        Expr::App(l, r) => Ok(NamelessExpr::App(
            Box::new(remove_names(scope, l)?),
            Box::new(remove_names(scope, r)?),
        )),
        Expr::Lam(s, _, e) => {
            scope.push(s);
            let e = remove_names(scope, e)?;
            scope.pop();
            Ok(NamelessExpr::Lam(Box::new(e)))
        }
        Expr::Lit(n) => Ok(NamelessExpr::Lit(*n)),
        Expr::Var(n) => match &**n {
            "+" => Ok(NamelessExpr::Plus),
            "-" => Ok(NamelessExpr::Minus),
            "*" => Ok(NamelessExpr::Times),
            "/" => Ok(NamelessExpr::Divide),
            "mod" => Ok(NamelessExpr::Mod),
            "print" => Ok(NamelessExpr::Print),
            _ => scope
                .iter()
                .rposition(|n2| n == n2)
                .map(|n| NamelessExpr::Var(scope.len() - n - 1))
                .ok_or_else(|| format!("Not found: {}", n)),
        },
    }
}
