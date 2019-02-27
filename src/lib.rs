//! [![Build Status](https://travis-ci.org/remexre/fstlc.svg?branch=master)](https://travis-ci.org/remexre/fstlc) [![Dependency Status](https://deps.rs/repo/github/remexre/fstlc/status.svg)](https://deps.rs/repo/github/remexre/fstlc)
//!
//! An STLC to [Forth386](https://github.com/remexre/forth386) compiler.

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(grammar);
mod tyck;

use std::{str::FromStr, sync::Arc};

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
