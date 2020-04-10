use crate::interpreter::value::Value;

use bumpalo::collections::{Vec, String};
use bumpalo::Bump;
use std::cell::RefCell;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Env<'a> {
    elems: &'a RefCell<Vec<'a, (String<'a>, Value<'a>)>>
}

impl<'a> Env<'a> {
    pub fn new(arena: &'a Bump) -> Env<'a> {
        Env {
            elems: arena.alloc(RefCell::new(Vec::new_in(arena)))
        }
    }

    pub fn add_value(&mut self, arena: &'a Bump, name: std::string::String, value: Value<'a>) {
        self.elems.borrow_mut().push((String::from_str_in(&name, arena), value));
    }

    pub fn set_value(&mut self, arena: &'a Bump, name: std::string::String, value: Value<'a>) {
        self.elems.borrow_mut().push((String::from_str_in(&name, arena), value));
    }

    pub fn get_value(&self, key: &str) -> Value<'a> {
        let vec = self.elems.borrow();
        for (k, v) in vec.iter() {
            if *k == key {
                return *v;
            }
        }
        return Value::Undefined { };
    }

    pub fn borrow_mut_value(&mut self, name: &str) -> &mut Value<'a> {
        unimplemented!()
        //self.elems.get_mut(name).expect("Name not found.")
    }
}
