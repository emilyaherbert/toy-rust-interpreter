use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::types::exp::Exp;
use crate::types::stmt::Stmt;

pub struct TestRunner {}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner {}
    }

    pub fn test(&self, ir: Vec<Stmt>, expected_output: Value) {
        let mut interpreter = Interpreter::new();
        let res = interpreter.eval(ir);
        assert_eq!(res, expected_output);
    }
}
