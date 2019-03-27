use crate::cam::StaticCombinator;

/// A static or dynamic combinator. This representation is for implementing evaluation by term
/// rewriting.
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Combinator {
    /// The `App` combinator. `App(Λ(x)y, z) = x(y, z)`
    #[display(fmt = "App")]
    App,

    /// Applies a combinator to another.
    #[display(fmt = "({} {})", _0, _1)]
    Apply(Box<Combinator>, Box<Combinator>),

    /// The composition combinator. `(x∘y)z = x(yz)`
    #[display(fmt = "({} ∘ {})", _0, _1)]
    Com(Box<Combinator>, Box<Combinator>),

    /// The dynamic pair combinator, `(,)`.
    #[display(fmt = "({}, {})", _0, _1)]
    DPair(Box<Combinator>, Box<Combinator>),

    /// The `Fst` combinator. `Fst(x, y) = x`
    #[display(fmt = "Fst")]
    Fst,

    /// The `Λ` combinator. `App(Λ(x)y, z) = x(y, z)`
    #[display(fmt = "Λ({})", _0)]
    Lam(Box<Combinator>),

    /// A bare name.
    #[display(fmt = "{}", _0)]
    Name(String),

    /// A bare number.
    #[display(fmt = "{}", _0)]
    Num(u32),

    /// The `<,>` combinator. `<x, y>z = (xz, yz)`
    #[display(fmt = "<{}, {}>", _0, _1)]
    Pair(Box<Combinator>, Box<Combinator>),

    /// The `'` combinator. `'xy = x`
    #[display(fmt = "'{}", _0)]
    Quote(Box<Combinator>),

    /// The `Snd` combinator. `Snd(x, y) = y`
    #[display(fmt = "Snd")]
    Snd,
}

impl Combinator {
    /// Attempts to evaluate the expression by "a few" steps. If the value is not actually modified,
    /// the evaluation has completed.
    pub fn eval_step(self) -> Combinator {
        match self {
            Combinator::Apply(l, r) => match (l.eval_step(), r.eval_step()) {
                (Combinator::Quote(x), _) => *x, // 'xy = x
                (Combinator::Com(ll, lr), r) => {
                    Combinator::Apply(ll, Box::new(Combinator::Apply(lr, Box::new(r)))) // (x∘y)z = x(yz)
                }
                (Combinator::Pair(ll, lr), r) => {
                    // <x, y>z = (xz, yz)
                    let r = Box::new(r);
                    Combinator::DPair(
                        Box::new(Combinator::Apply(ll, r.clone())),
                        Box::new(Combinator::Apply(lr, r)),
                    )
                }
                (Combinator::App, Combinator::DPair(rl, rr)) => match *rl {
                    Combinator::Apply(rll, rlr) => match *rll {
                        // App(Λ(x)y, z) = x(y, z)
                        Combinator::Lam(rll) => {
                            Combinator::Apply(rll, Box::new(Combinator::DPair(rlr, rr)))
                        }
                        rll => Combinator::Apply(
                            Box::new(Combinator::App),
                            Box::new(Combinator::DPair(
                                Box::new(Combinator::Apply(Box::new(rll), rlr)),
                                rr,
                            )),
                        ),
                    },
                    rl => Combinator::Apply(
                        Box::new(Combinator::App),
                        Box::new(Combinator::DPair(Box::new(rl), rr)),
                    ),
                },
                (Combinator::Fst, Combinator::DPair(x, _)) => *x, // Fst(x, y) = x
                (Combinator::Snd, Combinator::DPair(_, x)) => *x, // Snd(x, y) = y
                (l, r) => Combinator::Apply(Box::new(l), Box::new(r)),
            },

            // The rest of the cases (and the uncommented ones above) are all simply recursing down
            // to evaluate lower in the tree.
            Combinator::Com(l, r) => {
                Combinator::Com(Box::new(l.eval_step()), Box::new(r.eval_step()))
            }
            Combinator::DPair(l, r) => {
                Combinator::DPair(Box::new(l.eval_step()), Box::new(r.eval_step()))
            }
            Combinator::Pair(l, r) => {
                Combinator::Pair(Box::new(l.eval_step()), Box::new(r.eval_step()))
            }
            Combinator::Lam(c) => Combinator::Lam(Box::new(c.eval_step())),
            Combinator::Quote(c) => Combinator::Quote(Box::new(c.eval_step())),
            Combinator::App => Combinator::App,
            Combinator::Fst => Combinator::Fst,
            Combinator::Name(n) => Combinator::Name(n),
            Combinator::Num(n) => Combinator::Num(n),
            Combinator::Snd => Combinator::Snd,
        }
    }

    /// Evaluates a combinator expression fully.
    pub fn eval(mut self) -> Combinator {
        // App is a safe constant to make this a do-while -- if the combinator expression is just
        // App (and the loop is skipped as a result), the expression was irreducible anyway.
        loop {
            let last = self.clone();
            self = self.eval_step();
            if self == last {
                break;
            }
        }
        self
    }
}

impl From<StaticCombinator> for Combinator {
    fn from(c: StaticCombinator) -> Combinator {
        match c {
            StaticCombinator::App => Combinator::App,
            StaticCombinator::Com(l, r) => Combinator::Com(l.into(), r.into()),
            StaticCombinator::Fst => Combinator::Fst,
            StaticCombinator::Lam(b) => Combinator::Lam(b.into()),
            StaticCombinator::Pair(l, r) => Combinator::Pair(l.into(), r.into()),
            StaticCombinator::QuoteName(n) => Combinator::Quote(Box::new(Combinator::Name(n))),
            StaticCombinator::QuoteNum(n) => Combinator::Quote(Box::new(Combinator::Num(n))),
            StaticCombinator::Snd => Combinator::Snd,
        }
    }
}

impl From<StaticCombinator> for Box<Combinator> {
    fn from(c: StaticCombinator) -> Box<Combinator> {
        Box::new(c.into())
    }
}

impl From<Box<StaticCombinator>> for Combinator {
    fn from(c: Box<StaticCombinator>) -> Combinator {
        (*c).into()
    }
}

impl From<Box<StaticCombinator>> for Box<Combinator> {
    fn from(c: Box<StaticCombinator>) -> Box<Combinator> {
        (*c).into()
    }
}
