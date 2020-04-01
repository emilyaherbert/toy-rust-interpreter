use crate::interpreter::env::Env;
use crate::interpreter::stmtresult::{constructors::*, StmtResult};
use crate::interpreter::value::{constructors::*, Value};
use crate::types::exp::{constructors::*, Exp, Op2};
use crate::types::stmt::{constructors::*, LVal, Stmt};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval(&mut self, mut ir: &[Stmt], env: &mut Env) -> Value {
        match self.eval_stmts(ir, env) {
            StmtResult::Return { value } => value,
            _ => unimplemented!(),
        }
    }

    fn eval_stmts(&mut self, stmts: &[Stmt], env: &mut Env) -> StmtResult {
        let mut res = srnothing_();
        for s in stmts {
            res = self.eval_stmt(s, env);
        }
        res
    }

    fn eval_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> StmtResult {
        match stmt {
            Stmt::Let { name, named } => {
                let named2 = self.eval_exp(named, env);
                env.add_value(name, named2);
                srnothing_()
            }
            Stmt::Set { lval, named } => {
                let named = self.eval_exp(named, env);
                self.set_lval(lval, named, env);
                srnothing_()
            }
            Stmt::Return { value } => {
                let value2 = self.eval_exp(value, env);
                srreturn_(value2)
            }
            _ => unimplemented!(),
        }
    }

    fn eval_exp(&mut self, exp: &Exp, env: &mut Env) -> Value {
        match exp {
            Exp::Number { value } => vnumber_(*value),
            Exp::Identifier { name } => env.get_value(name),
            Exp::Array { exps } => {
                let mut values2: Vec<Value> = vec!();
                for v in exps {
                    values2.push(self.eval_exp(v, env));
                }
                varray_(values2)
            },
            Exp::Index { e1, e2 } => {
                let array = self.eval_exp(e1, env);
                let index = self.eval_exp(e2, env);
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

    fn set_lval(&mut self, lval: &LVal, rval: Value, env: &mut Env) {
        match lval {
            LVal::Identifier { name } => {
                env.set_value(name, rval);
            }
            LVal::Index { e, index } => {
                let name = self.get_id(e);
                let index_ = &self.eval_exp(index, env);
                let e_ = env.borrow_mut_value(&name);
                match e_ {
                    Value::Array { values } => {
                        if let Value::Number { value: y } = index_ {
                            let y_: f64 = *y;
                            if (y_ >= 0.0) && (y_ <= usize::max_value() as f64) {
                                values[y_ as usize] = rval;
                            }
                        } else {
                            unimplemented!("indexing with value {:?}", index_);
                        }
                    }
                    _ => (),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn get_id(&mut self, exp: &Exp) -> String {
        match exp {
            Exp::Identifier { name } => name.to_owned(),
            _ => "".to_owned(),
        }
    }
}
