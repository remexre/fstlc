use crate::Expr;

/// A nameless expression.
#[derive(Debug, Display, PartialEq)]
pub enum NamelessExpr {
    /// Function application.
    #[display(fmt = "({} {})", _0, _1)]
    App(Box<NamelessExpr>, Box<NamelessExpr>),

    /// Function abstraction.
    #[display(fmt = "λ{}", _0)]
    Lam(Box<NamelessExpr>),

    /// A literal value.
    #[display(fmt = "{}", _0)]
    Lit(u32),

    /// A globally named function.
    #[display(fmt = "{}", _0)]
    Nam(String),

    /// A variable reference.
    #[display(fmt = "${}", _0)]
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
        Expr::Var(n) => {
            let r = scope
                .iter()
                .rposition(|n2| n == n2)
                .map(|n| NamelessExpr::Var(scope.len() - n - 1));
            match r {
                Some(e) => Ok(e),
                None => match &**n {
                    "+" | "-" | "*" | "/" | "mod" | "print" => Ok(NamelessExpr::Nam(n.clone())),
                    n => Err(format!("Not found: {}", n)),
                },
            }
        }
    }
}
