#[cfg(test)]
mod tests {
    use crate::interpreter::value::constructors::*;
    use crate::test_runner::TestRunner;
    use crate::types::exp::constructors::*;
    use crate::types::stmt::{constructors::*, LVal};

    use bumpalo::Bump;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn let_test() {
        let ir = vec![let_("hi", number_(5.0)), return_(identifier_("hi"))];

        let expected_output = vnumber_(5.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
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
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }

    #[test]
    fn set_test2() {
        let ir = vec![
            let_("x", number_(0.0)),
            let_("y", identifier_("x")),
            set_(
                LVal::Identifier {
                    name: "y".to_string(),
                },
                number_(10.0),
            ),
            return_(identifier_("x")),
        ];

        let expected_output = vnumber_(0.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }

    #[test]
    fn arrays() {
        let ir = vec![
            let_("hi", array_(vec![number_(1.0), number_(2.0)])),
            return_(index_(identifier_("hi"), number_(1.0)))
        ];

        let expected_output = vnumber_(2.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }

    #[test]
    fn arrays2() {
        let ir = vec![
            let_("hi", array_(vec![number_(1.0), number_(2.0)])),
            set_(
                LVal::Index {
                    e: Box::new(identifier_("hi")),
                    index: Box::new(number_(1.0))
                },
                number_(10.0),
            ),
            return_(index_(identifier_("hi"), number_(1.0)))
        ];

        let expected_output = vnumber_(10.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }

    #[test]
    fn arrays3() {
        let ir = vec![
            let_("x", array_(vec![number_(1.0), number_(2.0)])),
            let_("y", identifier_("x")),
            set_(
                LVal::Index {
                    e: Box::new(identifier_("y")),
                    index: Box::new(number_(1.0))
                },
                number_(10.0),
            ),
            return_(index_(identifier_("x"), number_(1.0)))
        ];

        let expected_output = vnumber_(10.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }

    #[test]
    fn clos() {
        let ir = vec![
            let_("F", function_(vec!["x".to_string()], vec![
                return_(index_(identifier_("x"), number_(1.0)))
            ])),
            let_("foo", array_(vec![number_(1.0), number_(101.0)])),
            let_("bar", fun_app_(identifier_("F"), vec![identifier_("foo")])),
            return_(identifier_("bar"))
        ];

        let expected_output = vnumber_(101.0);

        let test_runner = TestRunner::new();
        let arena = Bump::new();
        test_runner.test(&arena, ir, expected_output);
    }
}
