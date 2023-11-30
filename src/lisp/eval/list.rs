use super::expr::Expr;
use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    car: Option<Rc<Expr>>,
    cdr: Option<Rc<Expr>>,
}

impl List {
    pub const NIL: List = List {
        car: None,
        cdr: None,
    };
    pub fn new(lhs: Option<Expr>, rhs: Option<Expr>) -> Self {
        List {
            car: lhs.map_or(None, |data| Some(Rc::new(data))),
            cdr: rhs.map_or(None, |data| Some(Rc::new(data))),
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
    pub fn atom(&self) -> bool {
        self.cdr().map_or(false, |x| match x {
            Expr::List(_) => false,
            _ => true,
        })
    }
}

impl Iterator for List {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let now = self.car.as_ref().map(|x| x.as_ref().clone());
        *self = self.cdr().map_or(List::NIL, |x| match x.clone() {
            Expr::List(l) => l.clone(),
            other => List::new(Some(other), None),
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
                    now = &next;
                }
                other => {
                    display.push_str(&(" . ".to_string() + &other.to_string()));
                    break;
                }
            }
        }
        display.push(')');

        write!(f, "{}", display.replace("()", "NIL"))

        // a fmt using Iterator but there is a problem that it dosen't have a dot in the dotted list
        // let mut display = String::new();
        // display.push('(');
        // for i in self.clone().into_iter() {
        //     display.push_str(&(' '.to_string() + &i.to_string()));
        // }
        // display.push(')');
        // write!(f, "{}", display.replace("( ", "(").replace("()", "NIL"))
    }
}
