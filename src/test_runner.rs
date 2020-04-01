use crate::interpreter::interpreter2::Interpreter;
use crate::interpreter::state::State;
use crate::interpreter::env::Env;
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
        let mut state = State::new();
        let mut env = Env::new();
        //let res = interpreter.eval(&ir, &mut state);
        let res = interpreter.eval(&ir, &mut env);
        assert_eq!(res, expected_output);
    }
}
