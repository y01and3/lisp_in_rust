use super::{expr::Expr, list::List};
use std::collections::HashMap;

struct Symbol {
    name: String,
    value: Expr,
    p_list: HashMap<String, Expr>,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol {
            name: name,
            value: Expr::List(List::NIL),
            p_list: HashMap::new(),
        }
    }
    pub fn set_value(&mut self, value:Expr) {
        self.value = value;
    }
    pub fn get_p_list(&self, prop: &String) -> Option<&Expr>{
        self.p_list.get(prop)
    }
    pub fn set_p_list(&mut self, prop: String, val:Expr){
        self.p_list.insert(prop, val);
    }
}
