use crate::interpreter::value::Value;

pub enum StmtResult<'a> {
    Return { value: Value<'a> },
    Nothing,
}

pub mod constructors {
    use crate::interpreter::stmtresult::StmtResult;
    use crate::interpreter::value::Value;

    pub fn srreturn_<'a>(value: Value<'a>) -> StmtResult<'a> {
        StmtResult::Return { value }
    }

    pub fn srnothing_<'a>() -> StmtResult<'a> {
        StmtResult::Nothing {}
    }
}
