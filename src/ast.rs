use std::{str::FromStr, sync::Arc};

#[derive(Debug, Display, Eq, PartialEq)]
pub enum Expr {
    #[display(fmt = "({} {})", _0, _1)]
    App(Box<Expr>, Box<Expr>),

    #[display(fmt = "(λ{}:{}. {})", _0, _1, _2)]
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
