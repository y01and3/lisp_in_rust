use super::expr::Expr;
use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
pub struct List {
    car: Option<Rc<Expr>>,
    cdr: Option<Rc<Expr>>,
}

impl List {
    pub const NIL: List = List {
        car: None,
        cdr: None,
    };
    pub fn new(data: Option<Expr>) -> Self {
        List {
            car: data.map_or(None, |data| Some(Rc::new(data))),
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

pub fn cons(obj1: Option<Expr>, obj2: Option<Expr>) -> Expr {
    let mut list = List::new(obj1);
    list.cdr = obj2.map(|x| Rc::new(x));
    Expr::List(Rc::new(list))
}

impl Iterator for List {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let now = self.car.as_ref().map(|x| x.as_ref().clone());
        *self = self.cdr().map_or(List::new(None), |x| match x.clone() {
            Expr::List(l) => l.as_ref().clone(),
            other => List::new(Some(other)),
        });
        now
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();

        display.push('(');
        display.push_str(&*self.car().map_or("".to_string(), |x| x.to_string()));

        let mut now = self;
        while let Some(next) = now.cdr() {
            match next {
                Expr::List(next) => {
                    display.push(' ');
                    display.push_str(&*next.car().map_or("".to_string(), |x| x.to_string()));
                    now = &**next;
                }
                other => {
                    display.push_str(&(" . ".to_string() + &other.to_string()));
                    break;
                }
            }
        }
        display.push(')');

        write!(f, "{}", display.replace("()", "NIL"))
    }
}
