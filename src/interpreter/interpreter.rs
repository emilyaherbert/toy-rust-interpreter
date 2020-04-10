use crate::interpreter::env::Env;
use crate::interpreter::stmtresult::{constructors::*, StmtResult};
use crate::interpreter::value::{constructors::*, Value};
use crate::types::exp::{Exp, Op2};
use crate::types::stmt::{LVal, Stmt};

use bumpalo::Bump;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn eval<'a>(&mut self, ir: &[Stmt], env: &mut Env<'a>, arena: &'a Bump) -> Value<'a> {
        match self.eval_stmts(ir, env, arena) {
            StmtResult::Return { value } => value,
            _ => unimplemented!(),
        }
    }

    fn eval_stmts<'a>(
        &mut self, stmts: &[Stmt], env: &mut Env<'a>, arena: &'a Bump,
    ) -> StmtResult<'a> {
        let mut res = srnothing_();
        for s in stmts {
            let r = self.eval_stmt(s, env, arena);
            res = r;
        }
        res
    }

    fn eval_stmt<'a>(&mut self, stmt: &Stmt, env: &mut Env<'a>, arena: &'a Bump) -> StmtResult<'a> {
        match stmt {
            Stmt::Let { name, named } => {
                let named = self.eval_exp(named, env, arena);
                match named {
                    Value::Array { values: _ } => env.add_value(arena, name.to_string(), named),
                    other => env.add_value(arena, name.to_string(), vref_(arena, other)),
                }
                srnothing_()
            }
            Stmt::Set { lval, named } => {
                let named = self.eval_exp(named, env, arena);
                self.set_lval(lval, named, env, arena);
                srnothing_()
            }
            Stmt::Return { value } => {
                let value = self.eval_exp(value, env, arena);
                srreturn_(value)
            }
            Stmt::Log { value } => {
                println!("--> {:?} == {:?}", value, self.eval_exp(value, env, arena));
                srnothing_()
            }
            _ => unimplemented!(),
        }
    }

    fn eval_exp<'a>(&mut self, exp: &Exp, env: &mut Env<'a>, arena: &'a Bump) -> Value<'a> {
        match exp {
            Exp::Number { value } => vnumber_(*value),
            Exp::Identifier { name } => {
                let v = env.get_value(name);
                match v {
                    Value::Ref { value } => value.get(),
                    other => other,
                }
            }
            Exp::BinOp { op, e1, e2 } => {
                let e1 = self.eval_exp(e1, env, arena);
                let e2 = self.eval_exp(e2, env, arena);
                match (op, e1, e2) {
                    (Op2::Add, Value::Number { value: v1 }, Value::Number { value: v2 }) => {
                        vnumber_(v1 + v2)
                    }
                    _ => unimplemented!(),
                }
            }
            Exp::Array { exps } => {
                let mut values: Vec<Value<'a>> = vec![];
                for v in exps {
                    let v = self.eval_exp(v, env, arena);
                    values.push(v);
                }
                varray_(arena, values)
            }
            Exp::Index { e1, e2 } => {
                let array = self.eval_exp(e1, env, arena);
                let index = self.eval_exp(e2, env, arena);
                match (array, index) {
                    (Value::Array { values }, Value::Number { value }) => {
                        if (value >= 0.0) && (value <= usize::max_value() as f64) {
                            values.borrow()[value as usize].to_owned()
                        } else {
                            vundefined_()
                        }
                    }
                    _ => panic!("Expected array and index."),
                }
            }
            Exp::Function { params, body } => vclos_(
                arena,
                arena.alloc(env.elems.clone()),
                params.to_vec(),
                body.to_vec(),
            ),
            Exp::FunApp { fun, fun_args } => {
                let clos = self.eval_exp(fun, env, arena);
                let mut fun_args2: Vec<Value> = vec![];
                for a in fun_args.into_iter() {
                    let a = self.eval_exp(&a, env, arena);
                    fun_args2.push(a);
                }
                match clos {
                    Value::Clos {
                        env: fun_env,
                        params,
                        body,
                    } => {
                        let mut fun_env = Env::new_with(fun_env.clone());
                        params
                            .into_iter()
                            .zip(fun_args2.into_iter())
                            .for_each(|(p, a)| {
                                fun_env.add_value(arena, p.to_string(), vref_(arena, a));
                            });
                        match self.eval_stmts(body, &mut fun_env, arena) {
                            StmtResult::Return { value } => value,
                            StmtResult::Nothing => vundefined_(),
                        }
                    }
                    _ => panic!("Expected env."),
                }
            }
        }
    }

    fn set_lval<'a>(&mut self, lval: &LVal, rval: Value<'a>, env: &mut Env<'a>, arena: &'a Bump) {
        match lval {
            LVal::Identifier { name } => {
                env.set_value(name.to_string(), rval);
            }
            LVal::Index { e, index } => {
                let name = self.get_id(e);
                match env.get_value(&name) {
                    Value::Array { values } => match self.eval_exp(index, env, arena) {
                        Value::Number { value } => {
                            if (value >= 0.0) && (value <= usize::max_value() as f64) {
                                std::mem::replace(&mut values.borrow_mut()[value as usize], rval);
                            }
                        }
                        _ => panic!("Expected number."),
                    },
                    _ => panic!("Expected array."),
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
