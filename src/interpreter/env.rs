use crate::interpreter::value::Value;

use bumpalo::collections::{String, Vec};
use bumpalo::Bump;

#[derive(PartialEq, Debug, Clone)]
pub struct Env<'a> {
    pub elems: Vec<'a, (String<'a>, Value<'a>)>,
}

impl<'a> Env<'a> {
    pub fn new(arena: &'a Bump) -> Env<'a> {
        Env {
            elems: Vec::new_in(arena),
        }
    }

    pub fn new_with(elems: Vec<'a, (String<'a>, Value<'a>)>) -> Env<'a> {
        Env { elems }
    }

    pub fn add_value(&mut self, arena: &'a Bump, name: std::string::String, value: Value<'a>) {
        self.elems.push((String::from_str_in(&name, arena), value));
    }

    pub fn set_value(&mut self, name: std::string::String, value: Value<'a>) {
        for (k, v) in self.elems.iter() {
            if k.clone() == name {
                match v {
                    Value::Ref { value: cell } => cell.set(value),
                    other => panic!("Expected ref, got {:?}", other),
                }
                return;
            }
        }
        panic!("Did not find match!!!!");
    }

    pub fn get_value(&self, key: &str) -> Value<'a> {
        for (k, v) in self.elems.iter() {
            if *k == key {
                return *v;
            }
        }
        return Value::Undefined {};
    }
}
