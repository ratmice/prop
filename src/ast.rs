use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Prop {
    True,
    And(Rc<Prop>, Rc<Prop>),
    Or(Rc<Prop>, Rc<Prop>),
    Neg(Rc<Prop>),
    Imp(Rc<Prop>, Rc<Prop>),
    Iff(Rc<Prop>, Rc<Prop>),
    Var(String),
}

#[derive(Debug)]
pub enum Binding {
    Bind(String, Rc<Expr>, Option<Typ>),
    Var(String, Option<Typ>),
}

#[derive(Debug)]
pub enum Typ {
    Typ(Prop),
}

#[derive(Debug)]
pub enum Expr {
    Lambda(Rc<Binding>, Rc<Expr>, Option<Typ>),
    Prop(Rc<Prop>),
    App(Rc<Expr>, Rc<Expr>),
}

impl fmt::Display for Typ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Typ::Typ(prop) => write!(f, "Typ({})", prop),
        }
    }
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Binding::Bind(s, expr, typ) => write!(f, "Bind({} : {:?} ≔ {})", s, typ, expr),
            Binding::Var(s, typ) => write!(f, "Var({}, {:?})", s, typ),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::App(e1, e2) => write!(f, "App({} {})", e1, e2),
            Expr::Prop(p) => write!(f, "Prop({})", p),
            Expr::Lambda(bind, expr, typ) => {
                write!(f, "Lambda((ⲗ {}. {}) : {:?})", bind, expr, typ)
            }
        }
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Prop::Neg(p) => write!(f, "Neg({})", p),
            Prop::True => write!(f, "⊤"),
            Prop::Iff(p1, p2) => write!(f, "Iff({} ↔ {})", p1, p2),
            Prop::And(p1, p2) => write!(f, "And({} ∧ {})", p1, p2),
            Prop::Or(p1, p2) => write!(f, "Or({} ∨ {})", p1, p2),
            Prop::Imp(p1, p2) => write!(f, "Imp({} → {})", p1, p2),
            Prop::Var(s) => write!(f, "Var({})", s),
        }
    }
}
