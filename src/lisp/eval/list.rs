use super::expr::Expr;
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub struct List {
    car: Option<Rc<Expr>>,
    cdr: Option<Rc<Expr>>,
}

impl List {
    pub fn new(data: Expr) -> Self {
        List {
            car: Some(Rc::new(data)),
            cdr: None,
        }
    }
    pub fn car(&self) -> Option<&Expr> {
        self.car.as_ref().map(|x| &**x)
    }
    pub fn cdr(&self) -> Option<&Expr> {
        self.cdr.as_ref().map(|x| &**x)
    }
    pub fn equal(&self, rhs: List) -> bool {
        self.to_string() == rhs.to_string()
    }
}

pub fn cons(obj1: Expr, obj2: Expr) -> Expr {
    let mut list = List::new(obj1);
    list.cdr = Some(Rc::new(obj2));
    Expr::List(Rc::new(list))
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();

        display.push('(');
        match self.car() {
            Some(car) => display.push_str(&car.to_string()),
            None => display.push_str("NIL"),
        }
        display.push(' ');
        let mut now = self;
        while let Some(next) = now.cdr() {
            match next {
                Expr::List(next) => {
                    display.push_str(&*next.to_string());
                    now = &**next;
                }
                other => {
                    display.push_str(&(". ".to_string() + &other.to_string()));
                    break;
                }
            }
        }
        display.push(')');
        display = display.replace(". )", ")");

        write!(f, "{}", display)
    }
}
