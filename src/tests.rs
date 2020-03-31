#[cfg(test)]
mod tests {
    use crate::interpreter::value::constructors::*;
    use crate::test_runner::TestRunner;
    use crate::types::exp::constructors::*;
    use crate::types::stmt::{constructors::*, LVal};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn let_test() {
        let ir = vec![let_("hi", number_(5.0)), return_(identifier_("hi"))];

        let expected_output = vnumber_(5.0);

        let test_runner = TestRunner::new();
        test_runner.test(ir, expected_output);
    }

    #[test]
    fn set_test() {
        let ir = vec![
            let_("hi", number_(5.0)),
            set_(
                LVal::Identifier {
                    name: "hi".to_string(),
                },
                number_(10.0),
            ),
            return_(identifier_("hi")),
        ];

        let expected_output = vnumber_(10.0);

        let test_runner = TestRunner::new();
        test_runner.test(ir, expected_output);
    }
}
