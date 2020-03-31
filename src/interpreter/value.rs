use crate::interpreter::state::Env;
use crate::types::stmt::Stmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
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
        values: Vec<Value>,
    },
    Clos {
        env: Env,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

pub mod constructors {
    use crate::interpreter::state::Env;
    use crate::interpreter::value::Value;
    use crate::types::stmt::Stmt;

    pub fn vnumber_(value: f64) -> Value {
        Value::Number { value }
    }

    pub fn vbool_(value: bool) -> Value {
        Value::Boolean { value }
    }

    pub fn vclos_(env: Env, params: Vec<String>, body: Vec<Stmt>) -> Value {
        Value::Clos { env, params, body }
    }

    pub fn varray_(values: Vec<Value>) -> Value {
        Value::Array { values }
    }
}
