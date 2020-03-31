use crate::interpreter::value::Value;

pub enum StmtResult {
    Return { value: Value },
    Nothing,
}

pub mod constructors {
    use crate::interpreter::stmtresult::StmtResult;
    use crate::interpreter::value::Value;

    pub fn srreturn_(value: Value) -> StmtResult {
        StmtResult::Return { value }
    }

    pub fn srnothing_() -> StmtResult {
        StmtResult::Nothing {}
    }
}
