use std::{rc::Rc, collections::HashMap};
use super::expr::Expr;

#[derive(Clone)]
pub struct Envirorment{
    outer: Option<Rc<Envirorment>>,
    vars: HashMap<String, Expr>
}

impl Envirorment{
    pub fn new(outer: Option<Rc<Envirorment>>) -> Self{
        Self{
            outer,
            vars: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: Expr){
        self.vars.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Expr>{
        match self.vars.get(key){
            Some(v) => Some(v.clone()),
            None => match &self.outer{
                Some(o) => o.get(key),
                None => None
            }
        }
    }

    pub fn get_outer(&self) -> Option<Envirorment>{
        self.outer.clone().map(|x| (*x).clone())
    }
}
