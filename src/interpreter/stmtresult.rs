use crate::interpreter::value::Value;

pub enum StmtResult {
    Break {
        label: Option<String>
    },
    Return {
        value: Value
    },
    Nothing
}

pub mod constructors {
    use crate::interpreter::stmtresult::StmtResult;
    use crate::interpreter::value::Value;

    pub fn srbreak_(label: Option<&str>) -> StmtResult {
        match label {
            Some(l) => StmtResult::Break {
                label: Some(l.to_string())
            },
            None => StmtResult::Break {
                label: None
            }
        }
    }

    pub fn srreturn_(value: Value) -> StmtResult {
        StmtResult::Return {
            value
        }
    }

    pub fn srnothing_() -> StmtResult {
        StmtResult::Nothing { }
    }
}