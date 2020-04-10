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

    pub fn eval<'a>(&mut self, ir: &[Stmt], env: Env<'a>, arena: &'a Bump) -> Value<'a> {
        match self.eval_stmts(ir, env, arena) {
            StmtResult::Return { value } => value,
            _ => unimplemented!(),
        }
    }

    fn eval_stmts<'a>(
        &mut self, stmts: &[Stmt], env: Env<'a>, arena: &'a Bump,
    ) -> StmtResult<'a> {
        let mut res = srnothing_();
        let mut env = env;
        for s in stmts {
            let (r, e) = self.eval_stmt(s, env, arena);
            res = r;
            env = e;
        }
        res
    }

    fn eval_stmt<'a>(
        &mut self, stmt: &Stmt, env: Env<'a>, arena: &'a Bump,
    ) -> (StmtResult<'a>, Env<'a>) {
        match stmt {
            Stmt::Let { name, named } => {
                let (named, mut env) = self.eval_exp(named, env, arena);
                match named {
                    Value::Array { values:_ } => env.add_value(arena, name.to_string(), named),
                    other => env.add_value(arena, name.to_string(), vref_(arena, other))
                }
                (srnothing_(), env)
            }
            Stmt::Set { lval, named } => {
                let (named, env) = self.eval_exp(named, env, arena);
                let env = self.set_lval(lval, named, env, arena);
                (srnothing_(), env)
            }
            Stmt::Return { value } => {
                let (value, env) = self.eval_exp(value, env, arena);
                (srreturn_(value), env)
            }
            _ => unimplemented!(),
        }
    }

    fn eval_exp<'a>(
        &mut self, exp: &Exp, env: Env<'a>, arena: &'a Bump,
    ) -> (Value<'a>, Env<'a>) {
        match exp {
            Exp::Number { value } => (vnumber_(*value), env),
            Exp::Identifier { name } => {
                let v = env.get_value(name);
                match v {
                    Value::Ref { value } => (value.get(), env),
                    other => (other, env)
                }
            },
            Exp::BinOp { op, e1, e2 } => {
                let (e1, env1) = self.eval_exp(e1, env, arena);
                let (e2, env2) = self.eval_exp(e2, env1, arena);
                match (op, e1, e2) {
                    (Op2::Add, Value::Number { value: v1 }, Value::Number { value: v2 }) => {
                        (vnumber_(v1 + v2), env2)
                    }
                    _ => unimplemented!(),
                }
            }
            Exp::Array { exps } => {
                let mut values: Vec<Value<'a>> = vec![];
                let mut env = env;
                for v in exps {
                    let (v, e) = self.eval_exp(v, env, arena);
                    values.push(v);
                    env = e;
                }
                (varray_(arena, values), env)
            }
            Exp::Index { e1, e2 } => {
                let (array, env1) = self.eval_exp(e1, env, arena);
                let (index, env2) = self.eval_exp(e2, env1, arena);
                match (array, index) {
                    (Value::Array { values }, Value::Number { value }) => {
                        if (value >= 0.0) && (value <= usize::max_value() as f64) {
                            (values.borrow()[value as usize].to_owned(), env2)
                        } else {
                            (vundefined_(), env2)
                        }
                    }
                    _ => panic!("Expected array and index."),
                }
            }
            Exp::Function { params, body } => (
                vclos_(arena, env.clone(), params.to_vec(), body.to_vec()),
                env,
            ),
            Exp::FunApp { fun, fun_args } => {
                let (clos, env) = self.eval_exp(fun, env, arena);
                let mut fun_args2: Vec<Value> = vec![];
                let mut env = env;
                for a in fun_args.into_iter() {
                    let (a, e) = self.eval_exp(&a, env, arena);
                    fun_args2.push(a);
                    env = e;
                }
                match clos {
                    Value::Clos {
                        env: fun_env,
                        params,
                        body,
                    } => {
                        let mut fun_env = fun_env.clone();
                        params
                            .into_iter()
                            .zip(fun_args2.into_iter())
                            .for_each(|(p, a)| {
                                fun_env.add_value(arena, p.to_string(), vref_(arena, a));
                            });
                        match self.eval_stmts(body, fun_env, arena) {
                            StmtResult::Return { value } => (value, env),
                            StmtResult::Nothing => (vundefined_(), env),
                        }
                    }
                    _ => panic!("Expected env."),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn set_lval<'a>(
        &mut self, lval: &LVal, rval: Value<'a>, mut env: Env<'a>, arena: &'a Bump,
    ) -> Env<'a> {
        match lval {
            LVal::Identifier { name } => {
                env.set_value(name.to_string(), rval);
                env
            }
            LVal::Index { e, index } => {
                let name = self.get_id(e);
                match env.get_value(&name) {
                    Value::Array { values } => match self.eval_exp(index, env, arena) {
                        (Value::Number { value }, env) => {
                            if (value >= 0.0) && (value <= usize::max_value() as f64) {
                                std::mem::replace(&mut values.borrow_mut()[value as usize], rval);
                            }
                            env
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
