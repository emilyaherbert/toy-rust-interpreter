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
    /*
    Clos {
        env: Env<'a>,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    */
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

    /*
    pub fn vclos_<'a>(env: Env<'a>, params: Vec<String>, body: Vec<Stmt>) -> Value<'a> {
        Value::Clos { env, params, body }
    }
    */

    pub fn varray_<'a>(arena: &'a Bump, values: std::vec::Vec<Value<'a>>) -> Value<'a> {
        let mut values2 = Vec::new_in(arena);
        for v in values {
            values2.push(v);
        }
        Value::Array { values: arena.alloc(RefCell::new(values2)) }
    }
}
