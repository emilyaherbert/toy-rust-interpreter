use crate::types::exp::Exp;

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    Block {
        body: Vec<Stmt>,
    },
    If {
        cond: Box<Exp>,
        true_part: Vec<Stmt>,
        false_part: Vec<Stmt>,
    },
    Let {
        name: String,
        named: Box<Exp>,
    },
    Set {
        lval: LVal,
        rval: Box<Exp>,
    },
    Return {
        value: Box<Exp>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum LVal {
    Identifier { name: String },
    Field { e: Box<Exp>, field: Box<Exp> },
    Index { e: Box<Exp>, index: Box<Exp> },
}

pub mod constructors {
    use crate::types::exp::Exp;
    use crate::types::stmt::LVal;
    use crate::types::stmt::Stmt;

    pub fn let_(name: &str, named: Exp) -> Stmt {
        Stmt::Let {
            name: name.to_string(),
            named: Box::new(named),
        }
    }

    pub fn set_(lval: LVal, rval: Exp) -> Stmt {
        Stmt::Set {
            lval,
            rval: Box::new(rval),
        }
    }

    pub fn return_(value: Exp) -> Stmt {
        Stmt::Return {
            value: Box::new(value),
        }
    }

    pub fn if_(cond: Exp, true_part: Vec<Stmt>, false_part: Vec<Stmt>) -> Stmt {
        Stmt::If {
            cond: Box::new(cond),
            true_part,
            false_part,
        }
    }

    pub fn block_(body: Vec<Stmt>) -> Stmt {
        Stmt::Block { body }
    }
}
