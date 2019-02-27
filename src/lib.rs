//! [![Build Status](https://travis-ci.org/remexre/fstlc.svg?branch=master)](https://travis-ci.org/remexre/fstlc) [![Dependency Status](https://deps.rs/repo/github/remexre/fstlc/status.svg)](https://deps.rs/repo/github/remexre/fstlc)
//!
//! An STLC to [Forth386](https://github.com/remexre/forth386) compiler.

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lalrpop_util;

mod compile;
lalrpop_mod!(grammar);
mod tyck;

pub use crate::compile::Chunk;
use std::{iter::once, str::FromStr, sync::Arc};

#[derive(Debug, Display, Eq, PartialEq)]
pub enum Expr {
    #[display(fmt = "({} {})", _0, _1)]
    App(Box<Expr>, Box<Expr>),
    #[display(fmt = "(\\{}:{}. {})", _0, _1, _2)]
    Lam(String, Arc<Type>, Box<Expr>),
    #[display(fmt = "{}", _0)]
    Lit(u32),
    #[display(fmt = "{}", _0)]
    Var(String),
}

impl Expr {
    /// Compiles an expression to Forth chunks.
    pub fn compile(&self) -> Result<impl Iterator<Item = Chunk>, String> {
        let mut scope = Vec::new();
        let nameless = crate::compile::remove_names(&mut scope, self)?;
        let mut hoist = Vec::new();
        let mut depth = 0;
        let main = crate::compile::compile(&nameless, &mut depth, &mut hoist);
        hoist.push(("main".to_string(), main, depth));

        Ok(hoist.into_iter().flat_map(|(name, body, depth)| {
            once(Chunk::Def(name))
                .chain(body)
                .chain(if depth == 0 {
                    None
                } else {
                    Some(Chunk::Discard(depth))
                })
                .chain(once(Chunk::EndDef))
        }))
    }
}

impl FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Expr, String> {
        crate::grammar::ExprParser::new()
            .parse(s)
            .map_err(|err| err.to_string())
    }
}

#[derive(Debug, Display, Eq, PartialEq)]
pub enum Type {
    #[display(fmt = "({}) -> {}", _0, _1)]
    Arr(Arc<Type>, Arc<Type>),
    #[display(fmt = "int")]
    Int,
}

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Type, String> {
        crate::grammar::TypeParser::new()
            .parse(s)
            .map_err(|err| err.to_string())
    }
}
