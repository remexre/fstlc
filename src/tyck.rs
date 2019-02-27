use crate::{Expr, Type};
use std::rc::Rc;

impl Expr {
    /// Typechecks the expression.
    pub fn tyck(&self) -> Result<Rc<Type>, String> {
        let mut scope = Vec::new();
        tyck(&mut scope, self)
    }
}

fn tyck<'e>(scope: &mut Vec<(&'e str, &'e Rc<Type>)>, expr: &'e Expr) -> Result<Rc<Type>, String> {
    match expr {
        Expr::App(l, r) => {
            let lt = tyck(scope, l)?;
            let rt = tyck(scope, r)?;
            match &*lt {
                Type::Arr(llt, lrt) => {
                    if llt == &rt {
                        Ok(lrt.clone())
                    } else {
                        Err(format!(
                            "{} expects an {}, but got {} (of type {})",
                            l, llt, r, rt
                        ))
                    }
                }
                _ => Err(format!("Not a function: {}", l)),
            }
        }
        Expr::Lam(s, t, e) => {
            scope.push((s, t));
            let et = tyck(scope, e)?;
            scope.pop();
            Ok(Rc::new(Type::Arr(t.clone(), et)))
        }
        Expr::Lit(_) => Ok(Rc::new(Type::Int)),
        Expr::Var(s) => scope
            .iter()
            .find(|(n, _)| n == s)
            .map(|(_, ty)| (*ty).clone())
            .ok_or_else(|| format!("Not found: {}", s)),
    }
}
