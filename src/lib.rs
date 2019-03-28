//! [![Build Status](https://travis-ci.org/remexre/fstlc.svg?branch=master)](https://travis-ci.org/remexre/fstlc) [![Dependency Status](https://deps.rs/repo/github/remexre/fstlc/status.svg)](https://deps.rs/repo/github/remexre/fstlc)
//!
//! An STLC to [Forth386](https://github.com/remexre/forth386) compiler.

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lalrpop_util;
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

#[cfg(test)]
pub(crate) use crate::cam::StaticCombinator;
pub use crate::{
    ast::{Expr, Type},
    cam::eval::Combinator as DynamicCombinator,
};

impl Expr {
    /// Compiles the expression to a sequence of Forth definitions. The names generated are prefixed
    /// with the given string.
    pub fn compile(&self, prefix: &str) -> Result<Vec<Vec<String>>, String> {
        self.to_combinators()
            .map(|combinators| combinators.to_forth(prefix))
    }
}
