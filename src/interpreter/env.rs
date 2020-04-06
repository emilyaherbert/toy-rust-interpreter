use crate::interpreter::value::Value;

use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Env<'a> {
    elems: HashMap<String, Value<'a>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            elems: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, name: &str, value: Value<'a>) {
        self.elems.insert(name.to_string(), value);
    }

    pub fn set_value(&mut self, name: &str, value: Value<'a>) {
        let addr = self.elems.get(name).expect("Name not found.");
        self.elems.insert(name.to_string(), value);
    }

    pub fn get_value(&self, name: &str) -> Value<'a> {
        self.elems
            .get(name)
            .map(|e| e.to_owned())
            .expect("Name not found.")
    }

    pub fn borrow_mut_value(&mut self, name: &str) -> &mut Value<'a> {
        self.elems.get_mut(name).expect("Name not found.")
    }
}
