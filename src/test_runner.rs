use crate::interpreter::env::Env;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::types::exp::Exp;
use crate::types::stmt::Stmt;

use bumpalo::Bump;

pub struct TestRunner {}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {}
    }

    pub fn test<'a>(&self, arena: &'a Bump, env: Env<'a>, ir: Vec<Stmt>) -> Value<'a> {
        let mut interpreter = Interpreter::new();
        let res = interpreter.eval(&ir, env, arena);
        res
    }
}
