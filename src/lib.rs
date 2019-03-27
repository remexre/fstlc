//! [![Build Status](https://travis-ci.org/remexre/fstlc.svg?branch=master)](https://travis-ci.org/remexre/fstlc) [![Dependency Status](https://deps.rs/repo/github/remexre/fstlc/status.svg)](https://deps.rs/repo/github/remexre/fstlc)
//!
//! An STLC to [Forth386](https://github.com/remexre/forth386) compiler.

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod ast;
mod cam;
lalrpop_mod!(grammar);
mod nameless;
#[cfg(test)]
mod tests;
mod tyck;

pub use crate::ast::{Expr, Type};
pub(crate) use crate::nameless::remove_names;
#[cfg(test)]
pub(crate) use crate::{
    cam::{eval::Combinator, StaticCombinator},
    nameless::NamelessExpr,
};

impl Expr {
    /// Compiles the expression to a sequence of Forth definitions. The names generated are prefixed
    /// with the given string.
    pub fn compile(&self, prefix: &str) -> Result<Vec<Vec<String>>, String> {
        let nameless = remove_names(&mut Vec::new(), self)?;
        info!("Nameless Representation: {}", nameless);
        let combinator = nameless.to_combinator();
        info!("Combinators: {}", combinator);
        Ok(combinator.to_forth(prefix))
    }
}
