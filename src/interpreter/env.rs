use crate::interpreter::value::Value;

use bumpalo::collections::{String, Vec};
use bumpalo::Bump;
use std::cell::Cell;

#[derive(PartialEq, Debug, Clone)]
pub struct Env<'a> {
    pub elems: Vec<'a, (String<'a>, &'a Cell<Value<'a>>)>,
}

impl<'a> Env<'a> {
    pub fn new(arena: &'a Bump) -> Env<'a> {
        Env {
            elems: Vec::new_in(arena),
        }
    }

    pub fn new_with(elems: Vec<'a, (String<'a>, &'a Cell<Value<'a>>)>) -> Env<'a> {
        Env { elems: elems }
    }

    pub fn add_value(&mut self, arena: &'a Bump, name: std::string::String, value: Value<'a>) {
        let value: &'a Cell<Value<'a>> = arena.alloc(Cell::new(value));
        self.elems.push((String::from_str_in(&name, arena), value));
    }

    pub fn set_value(&mut self, name: std::string::String, value: Value<'a>) {
        for (k, v) in self.elems.iter() {
            if k.clone() == name {
                v.set(value);
                return;
            }
        }
        panic!("Did not find match!!!!");
    }

    pub fn get_value(&self, key: &str) -> Value<'a> {
        for (k, v) in self.elems.iter() {
            if *k == key {
                return v.get();
            }
        }
        return Value::Undefined {};
    }
}
