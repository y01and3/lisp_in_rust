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
        self.clone() == rhs.clone()
    }
    pub fn atom(&self) -> bool {
        self.equal(List::NIL)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lisp::eval::num::{int::Int, number::Number, real::Real};

    #[test]
    fn test_list_car_cdr() {
        let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
        let list = Expr::List(List::new(Some(num.clone()), None));
        let listf = List::new(
            Some(Expr::List(List::NIL)),
            Some(Expr::List(List::new(
                Some(Expr::List(List::new(
                    Some(Expr::List(List::NIL)),
                    Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
                ))),
                Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
            ))),
        );
        assert_eq!(listf.car(), Some(&Expr::List(List::NIL)));
        assert_eq!(
            listf.cdr(),
            Some(&Expr::List(List::new(
                Some(Expr::List(List::new(
                    Some(Expr::List(List::NIL)),
                    Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
                ))),
                Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
            )))
        );
    }
    #[test]
    fn test_list_atom() {
        let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
        let list = List::new(Some(num.clone()), Some(num.clone()));

        assert_eq!(list.atom(), false);
        assert_eq!(List::NIL.atom(), true);
    }

    #[test]
    fn test_list_equal() {
        let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
        let list = Expr::List(List::new(Some(num.clone()), None));
        let listf = List::new(
            Some(Expr::List(List::NIL)),
            Some(Expr::List(List::new(
                Some(Expr::List(List::new(
                    Some(Expr::List(List::NIL)),
                    Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
                ))),
                Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
            ))),
        );
        assert_eq!(listf.equal(listf.clone()), true);
        assert_eq!(listf.equal(List::NIL), false);
        assert_eq!(listf.equal(List::new(Some(num.clone()), None)), false);
        assert_eq!(List::NIL.equal(List::new(None, None)), true);
    }

    #[test]
    fn test_list_iter() {
        let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
        let list = Expr::List(List::new(Some(num.clone()), None));
        let listf = List::new(
            Some(Expr::List(List::NIL)),
            Some(Expr::List(List::new(
                Some(Expr::List(List::new(
                    Some(Expr::List(List::NIL)),
                    Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
                ))),
                Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
            ))),
        );
        assert_eq!(
            listf
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            vec!["NIL", "(NIL (1) . 1)", "1", "1"]
        );
    }

    #[test]
    fn test_list_display() {
        let num = Expr::Number(Number::Real(Real::Int(Int::new(1))));
        let list = Expr::List(List::new(Some(num.clone()), None));
        let listf = List::new(
            Some(Expr::List(List::NIL)),
            Some(Expr::List(List::new(
                Some(Expr::List(List::new(
                    Some(Expr::List(List::NIL)),
                    Some(Expr::List(List::new(Some(list.clone()), Some(num.clone())))),
                ))),
                Some(Expr::List(List::new(Some(num.clone()), Some(list.clone())))),
            ))),
        );
        assert_eq!(listf.to_string(), "(NIL (NIL (1) . 1) 1 1)");
    }
}
