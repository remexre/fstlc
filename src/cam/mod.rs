//! Compiles the nameless lambda calculus to the Categorical Abstract Machine.

use crate::nameless::NamelessExpr;

/// A combinator that is compiled to.
#[derive(Debug, Display, PartialEq)]
pub enum StaticCombinator {
    /// The `App` combinator. `App(Λ(x)y, z) = x(y, z)`
    #[display(fmt = "App")]
    App,

    /// The composition combinator. `(xoy)z = x(yz)`
    ///
    /// TODO Unicode.
    #[display(fmt = "{} o {}", _0, _1)]
    Com(Box<StaticCombinator>, Box<StaticCombinator>),

    /// The `Fst` combinator. `Fst(x, y) = x`
    #[display(fmt = "Fst")]
    Fst,

    /// The `Λ` combinator. `App(Λ(x)y, z) = x(y, z)`
    #[display(fmt = "Λ({})", _0)]
    Lam(Box<StaticCombinator>),

    /// The `<,>` combinator. `<x,y>z = (xz,yz)`
    #[display(fmt = "<{},{}>", _0, _1)]
    Pair(Box<StaticCombinator>, Box<StaticCombinator>),

    /// The `'` combinator, specialized to a name. `'xy = x`
    #[display(fmt = "'{}", _0)]
    QuoteName(String),

    /// The `'` combinator, specialized to a number. `'xy = x`
    #[display(fmt = "'{}", _0)]
    QuoteNum(u32),

    /// The `Snd` combinator. `Snd(x, y) = y`
    #[display(fmt = "Snd")]
    Snd,
}

impl StaticCombinator {
    /// Compiles a `StaticCombinator` to a sequence of Forth words.
    pub fn to_forth(&self) -> Vec<String> {
        match *self {
            StaticCombinator::App => vec!["FSTLC-APP".to_string()],
            StaticCombinator::Com(ref l, ref r) => {
                let mut v = r.to_forth();
                v.extend(l.to_forth());
                v
            }
            StaticCombinator::Fst => vec!["FSTLC-FST".to_string()],
            StaticCombinator::Lam(ref b) => unimplemented!("Λ({})", b),
            StaticCombinator::Pair(ref l, ref r) => {
                let mut v = vec!["DUP".to_string()];
                v.extend(l.to_forth());
                v.push("SWAP".to_string());
                v.extend(r.to_forth());
                v.push("FSTLC-MAKE-PAIR".to_string());
                v
            }
            StaticCombinator::QuoteName(ref n) => {
                vec!["DROP".to_string(), "'".to_string(), format!("{}", n)]
            }
            StaticCombinator::QuoteNum(n) => vec!["DROP".to_string(), format!("{}", n)],
            StaticCombinator::Snd => vec!["FSTLC-SND".to_string()],
        }
    }
}

impl NamelessExpr {
    /// Compiles a `NamelessExpr` into a `StaticCombinator`.
    pub fn to_combinator(&self) -> StaticCombinator {
        match *self {
            NamelessExpr::App(ref l, ref r) => StaticCombinator::Com(
                Box::new(StaticCombinator::App),
                Box::new(StaticCombinator::Pair(
                    Box::new(l.to_combinator()),
                    Box::new(r.to_combinator()),
                )),
            ),
            NamelessExpr::Lam(ref b) => StaticCombinator::Lam(Box::new(b.to_combinator())),
            NamelessExpr::Lit(n) => StaticCombinator::QuoteNum(n),
            NamelessExpr::Nam(ref n) => StaticCombinator::QuoteName(n.clone()),
            NamelessExpr::Var(0) => StaticCombinator::Snd,
            NamelessExpr::Var(n) => StaticCombinator::Com(
                Box::new(NamelessExpr::Var(n - 1).to_combinator()),
                Box::new(StaticCombinator::Fst),
            ),
        }
    }
}
