use crate::types::stmt::{constructors::*, LVal, Stmt};
use crate::types::exp::{constructors::*, Exp, Op2};
use crate::interpreter::value::{constructors::*, Value};
use crate::interpreter::stmtresult::{constructors::*, StmtResult};
use crate::interpreter::state::State;

pub struct Interpreter {
    state: State,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            state: State::new(),
        }
    }

    pub fn eval(&mut self, mut ir: Vec<Stmt>, input: Exp) -> Value {
        unimplemented!()
    }

    fn eval_stmts(&mut self, stmts: &[Stmt]) -> StmtResult {
        unimplemented!()
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        unimplemented!()
    }

    fn eval_exp(&mut self, exp: &Exp) -> Value {
        unimplemented!()
    }

}
