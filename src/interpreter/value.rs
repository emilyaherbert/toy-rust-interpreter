use crate::interpreter::env::Env;
use crate::types::stmt::Stmt;

use bumpalo::collections::{String, Vec};
use std::cell::{Cell, RefCell};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Value<'a> {
    Undefined {},
    Number {
        value: f64,
    },
    Boolean {
        value: bool,
    },
    Identifier {
        name: &'a String<'a>,
    },
    Array {
        values: &'a RefCell<Vec<'a, Value<'a>>>,
    },
    Clos {
        env: Env<'a>,
        params: &'a Vec<'a, String<'a>>,
        body: &'a Vec<'a, Stmt>,
    },
    Ref {
        value: &'a Cell<Value<'a>>,
    },
}

pub mod constructors {
    use crate::interpreter::env::Env;
    use crate::interpreter::value::Value;
    use crate::types::stmt::Stmt;

    use bumpalo::collections::{String, Vec};
    use bumpalo::Bump;
    use std::cell::{Cell, RefCell};

    pub fn vundefined_<'a>() -> Value<'a> {
        Value::Undefined {}
    }

    pub fn vnumber_<'a>(value: f64) -> Value<'a> {
        Value::Number { value }
    }

    pub fn vbool_<'a>(value: bool) -> Value<'a> {
        Value::Boolean { value }
    }

    pub fn vclos_<'a>(
        arena: &'a Bump, env: Env<'a>, params: std::vec::Vec<std::string::String>,
        body: std::vec::Vec<Stmt>,
    ) -> Value<'a> {
        let mut params2 = Vec::new_in(arena);
        for p in params {
            params2.push(String::from_str_in(&p, arena));
        }
        let mut body2 = Vec::new_in(arena);
        for b in body {
            body2.push(b);
        }
        Value::Clos {
            env,
            params: arena.alloc(params2),
            body: arena.alloc(body2),
        }
    }

    pub fn varray_<'a>(arena: &'a Bump, values: std::vec::Vec<Value<'a>>) -> Value<'a> {
        let mut values2 = Vec::new_in(arena);
        for v in values {
            values2.push(v);
        }
        Value::Array {
            values: arena.alloc(RefCell::new(values2)),
        }
    }

    pub fn vref_<'a>(arena: &'a Bump, value: Value<'a>) -> Value<'a> {
        Value::Ref {
            value: arena.alloc(Cell::new(value)),
        }
    }
}
