use super::expr::Expr;
use std::{collections::HashMap, fmt::Display, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    name: String,
    value: Option<Rc<Expr>>,
    p_list: HashMap<String, Rc<Expr>>,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol {
            name,
            value: None,
            p_list: HashMap::new(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn value(&self) -> Option<&Expr> {
        self.value.as_ref().map(|x| &**x)
    }
    pub fn set_value(&mut self, value: Expr) {
        self.value = Some(Rc::new(value));
    }
    pub fn get_p_list(&self, prop: &String) -> Option<&Expr> {
        self.p_list.get(prop).map(|x| &**x)
    }
    pub fn set_p_list(&mut self, prop: String, val: Expr) {
        self.p_list.insert(prop, Rc::new(val));
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
