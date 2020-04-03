use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::env::Env;
use crate::interpreter::value::Value;
use crate::types::exp::Exp;
use crate::types::stmt::Stmt;

use bumpalo::Bump;

pub struct TestRunner {}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {}
    }

    pub fn test<'a>(&self, arena: &'a Bump, ir: Vec<Stmt>, expected_output: Value) {
        let mut interpreter = Interpreter::new();
        let mut env = Env::new();
        let res = interpreter.eval(&ir, &mut env, &arena);
        //assert_eq!(res, expected_output);
    }
}
