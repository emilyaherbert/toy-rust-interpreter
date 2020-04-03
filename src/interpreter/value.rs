use crate::interpreter::env::Env;
use crate::types::stmt::Stmt;

use std::cell::RefCell;
use bumpalo::collections::Vec;

#[derive(PartialEq, Debug, Clone)]
pub enum Value<'a> {
    Undefined { },
    Number {
        value: f64,
    },
    Boolean {
        value: bool,
    },
    Identifier {
        name: String,
    },
    Array {
        values: &'a RefCell<Vec<'a, Value<'a>>>
    },
    Clos {
        env: Env<'a>,
        params: &'a Vec<'a, String>,
        body: &'a Vec<'a, Stmt>,
    },
}

pub mod constructors {
    //use crate::interpreter::state::Env;
    use crate::interpreter::value::Value;
    use crate::types::stmt::Stmt;
    use crate::interpreter::env::Env;

    use std::cell::RefCell;
    use rc::Rc;
    use bumpalo::{
        Bump,
        collections::Vec
    };

    pub fn vundefined_<'a>() -> Value<'a> {
        Value::Undefined { }
    }

    pub fn vnumber_<'a>(value: f64) -> Value<'a> {
        Value::Number { value }
    }

    pub fn vbool_<'a>(value: bool) -> Value<'a> {
        Value::Boolean { value }
    }

    pub fn vclos_<'a>(arena: &'a Bump, env: Env<'a>, params: std::vec::Vec<String>, body: std::vec::Vec<Stmt>) -> Value<'a> {
        let mut params2 = Vec::new_in(arena);
        for p in params {
            params2.push(p);
        }
        let mut body2 = Vec::new_in(arena);
        for b in body {
            body2.push(b);
        }
        Value::Clos { env, params: arena.alloc(params2), body: arena.alloc(body2) }
    }

    pub fn varray_<'a>(arena: &'a Bump, values: std::vec::Vec<Value<'a>>) -> Value<'a> {
        let mut values2 = Vec::new_in(arena);
        for v in values {
            values2.push(v);
        }
        Value::Array { values: arena.alloc(RefCell::new(values2)) }
    }
}
