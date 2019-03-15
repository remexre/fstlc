use crate::{
    ast::{Expr, Type},
    cam::StaticCombinator,
    nameless::{remove_names, NamelessExpr},
};
use std::sync::Arc;

/// Compiles `const (id 42) 137`, checking the state through each step of the process.
#[test]
fn const_id_42_137() {
    const SRC: &str = "(λx:int. λy:int. x) ((λx:int. x) 42) 137";

    let expr = SRC.parse::<Expr>().unwrap();
    assert_eq!(
        expr,
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Lam(
                    "x".to_string(),
                    Arc::new(Type::Int),
                    Box::new(Expr::Lam(
                        "y".to_string(),
                        Arc::new(Type::Int),
                        Box::new(Expr::Var("x".to_string())),
                    )),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Lam(
                        "x".to_string(),
                        Arc::new(Type::Int),
                        Box::new(Expr::Var("x".to_string())),
                    )),
                    Box::new(Expr::Lit(42))
                )),
            )),
            Box::new(Expr::Lit(137)),
        )
    );
    assert_eq!(
        expr.to_string(),
        "(((λx:int. (λy:int. x)) ((λx:int. x) 42)) 137)"
    );

    let nameless = remove_names(&mut Vec::new(), &expr).unwrap();
    assert_eq!(
        nameless,
        NamelessExpr::App(
            Box::new(NamelessExpr::App(
                Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Lam(Box::new(
                    NamelessExpr::Var(1)
                ),)),)),
                Box::new(NamelessExpr::App(
                    Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Var(0)),)),
                    Box::new(NamelessExpr::Lit(42))
                )),
            )),
            Box::new(NamelessExpr::Lit(137)),
        )
    );

    let combinator = nameless.to_combinator();
    assert_eq!(
        combinator,
        StaticCombinator::Com(
            Box::new(StaticCombinator::App),
            Box::new(StaticCombinator::Pair(
                Box::new(StaticCombinator::Com(
                    Box::new(StaticCombinator::App),
                    Box::new(StaticCombinator::Pair(
                        Box::new(StaticCombinator::Lam(Box::new(StaticCombinator::Lam(
                            Box::new(StaticCombinator::Com(
                                Box::new(StaticCombinator::Snd),
                                Box::new(StaticCombinator::Fst),
                            )),
                        )))),
                        Box::new(StaticCombinator::Com(
                            Box::new(StaticCombinator::App),
                            Box::new(StaticCombinator::Pair(
                                Box::new(StaticCombinator::Lam(Box::new(StaticCombinator::Snd))),
                                Box::new(StaticCombinator::QuoteNum(42)),
                            )),
                        )),
                    )),
                )),
                Box::new(StaticCombinator::QuoteNum(137)),
            )),
        )
    );

    let forth = combinator.to_forth("test");
    assert_eq!(forth, &[[":", "fstlc-test-main", "0", ";"]]);
}
use crate::{
    ast::{Expr, Type},
    cam::StaticCombinator,
    nameless::{remove_names, NamelessExpr},
};
use std::sync::Arc;

/// Compiles `const (id 42) 137`, checking the state through each step of the process.
#[test]
fn const_id_42_137() {
    const SRC: &str = "(λx:int. λy:int. x) ((λx:int. x) 42) 137";

    let expr = SRC.parse::<Expr>().unwrap();
    assert_eq!(
        expr,
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Lam(
                    "x".to_string(),
                    Arc::new(Type::Int),
                    Box::new(Expr::Lam(
                        "y".to_string(),
                        Arc::new(Type::Int),
                        Box::new(Expr::Var("x".to_string())),
                    )),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Lam(
                        "x".to_string(),
                        Arc::new(Type::Int),
                        Box::new(Expr::Var("x".to_string())),
                    )),
                    Box::new(Expr::Lit(42))
                )),
            )),
            Box::new(Expr::Lit(137)),
        )
    );
    assert_eq!(
        expr.to_string(),
        "(((λx:int. (λy:int. x)) ((λx:int. x) 42)) 137)"
    );

    let nameless = remove_names(&mut Vec::new(), &expr).unwrap();
    assert_eq!(
        nameless,
        NamelessExpr::App(
            Box::new(NamelessExpr::App(
                Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Lam(Box::new(
                    NamelessExpr::Var(1)
                ),)),)),
                Box::new(NamelessExpr::App(
                    Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Var(0)),)),
                    Box::new(NamelessExpr::Lit(42))
                )),
            )),
            Box::new(NamelessExpr::Lit(137)),
        )
    );

    let combinator = nameless.to_combinator();
    assert_eq!(
        combinator,
        StaticCombinator::Com(
            Box::new(StaticCombinator::App),
            Box::new(StaticCombinator::Pair(
                Box::new(StaticCombinator::Com(
                    Box::new(StaticCombinator::App),
                    Box::new(StaticCombinator::Pair(
                        Box::new(StaticCombinator::Lam(Box::new(StaticCombinator::Lam(
                            Box::new(StaticCombinator::Com(
                                Box::new(StaticCombinator::Snd),
                                Box::new(StaticCombinator::Fst),
                            )),
                        )))),
                        Box::new(StaticCombinator::Com(
                            Box::new(StaticCombinator::App),
                            Box::new(StaticCombinator::Pair(
                                Box::new(StaticCombinator::Lam(Box::new(StaticCombinator::Snd))),
                                Box::new(StaticCombinator::QuoteNum(42)),
                            )),
                        )),
                    )),
                )),
                Box::new(StaticCombinator::QuoteNum(137)),
            )),
        )
    );

    let forth = combinator.to_forth("test");
    assert_eq!(forth, &[[":", "fstlc-test-main", "0", ";"]]);
}
