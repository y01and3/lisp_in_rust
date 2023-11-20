use super::expr::Expr;
use std::fmt::Display;

pub struct List {
    data: Expr,
    pointer: Option<Box<List>>,
}

impl List {
    pub fn new(data: Expr) -> Self {
        List {
            data,
            pointer: None,
        }
    }
    pub fn car(&self) -> Option<&Expr> {
        Some(&self.data)
    }
    pub fn cdr(&self) -> Option<&List> {
        self.pointer.as_ref().map(|x| &**x)
    }
}

pub fn cons(obj1: Expr, obj2: Expr) -> List {
    let mut list = List::new(obj1);
    match obj2 {
        Expr::List(cdr) => {
            list.pointer = Some(cdr);
        }
        Expr::Nil => {
            list.pointer = None;
        }
        _ => {
            list.pointer = Some(Box::new(List::new(obj2)));
        }
    }
    list
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut show = String::new();
        show.push_str("(");
        show.push_str(&self.data.to_string());
        let now = self;
        while let Some(next) = &now.pointer {
            let now = next;
            show.push_str(" ");
            show.push_str(&now.data.to_string());
        }
        show.push_str(")");
        write!(f, "{}", show)
    }
}
