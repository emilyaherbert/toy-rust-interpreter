use crate::interpreter::state::State;
use crate::interpreter::stmtresult::{constructors::*, StmtResult};
use crate::interpreter::value::{constructors::*, Value};
use crate::types::exp::{constructors::*, Exp, Op2};
use crate::types::stmt::{constructors::*, LVal, Stmt};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, mut ir: &[Stmt], state: &mut State) -> Value {
        match self.eval_stmts(ir, state) {
            StmtResult::Return { value } => value,
            _ => unimplemented!(),
        }
    }

    fn eval_stmts(&mut self, stmts: &[Stmt], state: &mut State) -> StmtResult {
        let mut res = srnothing_();
        for s in stmts {
            res = self.eval_stmt(s, state);
        }
        res
    }

    fn eval_stmt(&mut self, stmt: &Stmt, state: &mut State) -> StmtResult {
        match stmt {
            Stmt::Let { name, named } => {
                let named2 = self.eval_exp(named, state);
                state.add_value(name, named2);
                srnothing_()
            }
            Stmt::Set { lval, named } => match lval {
                LVal::Identifier { name } => {
                    let named2 = self.eval_exp(named, state);
                    state.set_value(name, named2);
                    srnothing_()
                }
                _ => unimplemented!(),
            },
            Stmt::Return { value } => {
                let value2 = self.eval_exp(value, state);
                srreturn_(value2)
            }
            _ => unimplemented!(),
        }
    }

    fn eval_exp(&mut self, exp: &Exp, state: &mut State) -> Value {
        match exp {
            Exp::Number { value } => vnumber_(*value),
            Exp::Identifier { name } => state.get_value(name),
            Exp::Array { exps } => {
                let mut values2: Vec<Value> = vec!();
                for v in exps {
                    values2.push(self.eval_exp(v, state));
                }
                varray_(values2)
            },
            Exp::Index { e1, e2 } => {
                let array = self.eval_exp(e1, state);
                let index = self.eval_exp(e2, state);
                match (array, index) {
                    (Value::Array { values }, Value::Number { value }) => {
                        if (value >= 0.0) && (value <= usize::max_value() as f64) {
                            values[value as usize].to_owned()
                        } else {
                            vundefined_()
                        }
                    },
                    _ => panic!("Expected array and index.")
                }
            }
            _ => unimplemented!(),
        }
    }
}
