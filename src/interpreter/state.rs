use crate::interpreter::value::Value;

use std::collections::HashMap;

pub type Env = HashMap<String, usize>;
pub type Store = HashMap<usize, Value>;

pub struct State {
    pub env: Env,
    pub store: Store,
    next_addr: usize,
}

impl State {
    pub fn new() -> State {
        State {
            env: HashMap::new(),
            store: HashMap::new(),
            next_addr: 0,
        }
    }

    pub fn add_value(&mut self, name: &str, value: Value) {
        let addr = self.next_addr;
        self.next_addr += 1;
        self.env.insert(name.to_string(), addr);
        self.store.insert(addr, value);
    }

    pub fn set_value(&mut self, name: &str, value: Value) {
        let addr = self.env.get(name).expect("Name not found.");
        self.store.insert(*addr, value);
    }

    pub fn get_value(&self, name: &str) -> Value {
        let addr = self.env.get(name).expect("Name not found.");
        let v = self.store.get(addr).expect("Addr not found.");
        v.to_owned()
    }

    pub fn borrow_mut_value(&mut self, name: &str) -> &mut Value {
        let addr = self.env.get(name).expect("Name not found.");
        let v = self.store.get_mut(addr).expect("Addr not found.");
        v
    }
}
