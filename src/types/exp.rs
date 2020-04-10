use crate::types::stmt::Stmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Exp {
    Number {
        value: f64,
    },
    Identifier {
        name: String,
    },
    BinOp {
        op: Op2,
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Array {
        exps: Vec<Exp>,
    },
    Index {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    FunApp {
        fun: Box<Exp>,
        fun_args: Vec<Exp>,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Op2 {
    Add,
}

pub mod constructors {
    use crate::types::exp::{Exp, Op2};
    use crate::types::stmt::Stmt;

    pub fn number_(value: f64) -> Exp {
        Exp::Number { value }
    }

    pub fn identifier_(id: &str) -> Exp {
        Exp::Identifier {
            name: id.to_string(),
        }
    }

    pub fn binop_(op: Op2, e1: Exp, e2: Exp) -> Exp {
        Exp::BinOp {
            op,
            e1: Box::new(e1),
            e2: Box::new(e2),
        }
    }

    pub fn function_(params: Vec<String>, body: Vec<Stmt>) -> Exp {
        Exp::Function { params, body }
    }

    pub fn array_(arr: Vec<Exp>) -> Exp {
        Exp::Array { exps: arr }
    }

    pub fn index_(e1: Exp, e2: Exp) -> Exp {
        Exp::Index {
            e1: Box::new(e1),
            e2: Box::new(e2),
        }
    }

    pub fn fun_app_(fun: Exp, fun_args: Vec<Exp>) -> Exp {
        Exp::FunApp {
            fun: Box::new(fun),
            fun_args,
        }
    }
}
