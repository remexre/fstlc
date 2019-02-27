use crate::{Expr, Type};
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref BUILTINS: Vec<(&'static str, &'static Arc<Type>)> = vec![
        ("+", &INT_TO_INT_TO_INT),
        ("-", &INT_TO_INT_TO_INT),
        ("*", &INT_TO_INT_TO_INT),
        ("/", &INT_TO_INT_TO_INT),
        ("mod", &INT_TO_INT_TO_INT),
        ("print", &INT_TO_INT),
    ];
    static ref INT: Arc<Type> = Arc::new(Type::Int);
    static ref INT_TO_INT: Arc<Type> = Arc::new(Type::Arr(INT.clone(), INT.clone()));
    static ref INT_TO_INT_TO_INT: Arc<Type> = Arc::new(Type::Arr(INT.clone(), INT_TO_INT.clone()));
}

impl Expr {
    /// Typechecks the expression.
    pub fn tyck(&self) -> Result<Arc<Type>, String> {
        let mut scope = BUILTINS.clone();
        tyck(&mut scope, self)
    }
}

fn tyck<'e>(
    scope: &mut Vec<(&'e str, &'e Arc<Type>)>,
    expr: &'e Expr,
) -> Result<Arc<Type>, String> {
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
            Ok(Arc::new(Type::Arr(t.clone(), et)))
        }
        Expr::Lit(_) => Ok(Arc::new(Type::Int)),
        Expr::Var(s) => scope
            .iter()
            .find(|(n, _)| n == s)
            .map(|(_, ty)| (*ty).clone())
            .ok_or_else(|| format!("Not found: {}", s)),
    }
}
