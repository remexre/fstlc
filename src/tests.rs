use crate::{remove_names, Combinator, Expr, NamelessExpr, StaticCombinator, Type};
use std::sync::Arc;

/// Compiles `id 1337`, checking the state through each step of the process.
#[test]
fn id_1337() {
    const SRC: &str = "(λx:int. x) 1337";

    let expr = SRC.parse::<Expr>().unwrap();
    assert_eq!(
        expr,
        Expr::App(
            Box::new(Expr::Lam(
                "x".to_string(),
                Arc::new(Type::Int),
                Box::new(Expr::Var("x".to_string())),
            )),
            Box::new(Expr::Lit(1337)),
        )
    );
    assert_eq!(expr.to_string(), "((λx:int. x) 1337)");

    let nameless = remove_names(&mut Vec::new(), &expr).unwrap();
    assert_eq!(
        nameless,
        NamelessExpr::App(
            Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Var(0)))),
            Box::new(NamelessExpr::Lit(1337)),
        )
    );

    let combinator = nameless.to_combinator();
    assert_eq!(
        combinator,
        StaticCombinator::Com(
            Box::new(StaticCombinator::App),
            Box::new(StaticCombinator::Pair(
                Box::new(StaticCombinator::Lam(Box::new(StaticCombinator::Snd))),
                Box::new(StaticCombinator::QuoteNum(1337)),
            )),
        )
    );

    let forth = combinator.to_forth("test");
    assert_eq!(
        forth,
        &[
            &[":", "fstlc-lambda-test-0", "FSTLC-SND", ";"] as &[_],
            &[
                ":",
                "fstlc-test-main",
                "0",
                "DUP",
                "'",
                "fstlc-lambda-test-0",
                "CFA",
                "SWAP",
                "DROP",
                "1337",
                "FSTLC-MAKE-PAIR",
                "FSTLC-APP",
                ";"
            ] as &[_],
        ]
    );

    let combinator = Combinator::Apply(combinator.into(), Box::new(Combinator::Num(0)));
    let c = combinator.clone();
    assert_eq!(c.to_string(), "((App ∘ <Λ(Snd),'1337>) 0)");

    let c = c.eval_step();
    assert_eq!(c.to_string(), "(App (<Λ(Snd),'1337> 0))");

    let c = c.eval_step();
    assert_eq!(c.to_string(), "(Snd (0,('1337 0)))");

    let c = c.eval_step();
    assert_eq!(c.to_string(), "1337");
    assert_eq!(c, Combinator::Num(1337));

    assert_eq!(combinator.eval(), Combinator::Num(1337));
}

/// Compiles `const (id 42) 137`, checking the state through each step of the process.
#[ignore]
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
                ))))),
                Box::new(NamelessExpr::App(
                    Box::new(NamelessExpr::Lam(Box::new(NamelessExpr::Var(0)))),
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
