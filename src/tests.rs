#[cfg(test)]
mod tests {
    use crate::declarator::{ExpressionDeclarator, HIGH_ORDER, LOW_ORDER, PrimitiveHandler};

    struct IntHandler {}

    impl PrimitiveHandler<i32> for IntHandler {
        fn from_string(&self, input: &String) -> Result<i32, ()> {
            let result = input.parse::<i32>();
            return if result.is_ok() {
                Result::Ok(result.unwrap())
            } else {
                Result::Err(())
            };
        }

        fn can_start_with(&self, input: String) -> bool {
            input.chars().all(|it| { it.is_numeric() })
        }
    }

    #[test]
    fn it_calculate() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            LOW_ORDER,
        );
        calculator.add_infix(
            "-".to_string(),
            "-".to_string(),
            Box::new(|op1, op2| { op1 - op2 }),
            LOW_ORDER,
        );

        assert_eq!(calculator.calculate("2+3 -7").ok().unwrap(), -2)
    }

    #[test]
    fn it_respect_order() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            HIGH_ORDER,
        );
        calculator.add_infix(
            "/".to_string(),
            "/".to_string(),
            Box::new(|op1, op2| { op1 / op2 }),
            LOW_ORDER,
        );

        assert_eq!(calculator.calculate("4+6/2").ok().unwrap(), 5)
    }

    #[test]
    fn it_respect_parenthesis() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            HIGH_ORDER,
        );
        calculator.add_infix(
            "/".to_string(),
            "/".to_string(),
            Box::new(|op1, op2| { op1 / op2 }),
            LOW_ORDER,
        );

        assert_eq!(calculator.calculate("4+(6/2)").ok().unwrap(), 7);
        assert_eq!(calculator.calculate("(((1))+(1))").ok().unwrap(), 2);
    }

    #[test]
    fn it_fails_on_unknown_token() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            HIGH_ORDER,
        );
        let result = calculator.calculate("4+(6/2)");
        assert_eq!(result.is_err(), true);

        let token = result.err().unwrap();
        assert_eq!(token.get_pos(), 4);
        assert_eq!(token.get_value(), "/".to_string())
    }

    #[test]
    fn it_fails_on_wrong_primitive() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            HIGH_ORDER,
        );

        let result = calculator.calculate("4+d");

        assert_eq!(result.is_err(), true);

        let token = result.err().unwrap();
        assert_eq!(token.get_pos(), 2);
        assert_eq!(token.get_value(), "d".to_string())
    }

    #[test]
    fn it_fails_on_wrong_parenthesis() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_infix(
            "+".to_string(),
            "+".to_string(),
            Box::new(|op1, op2| { op1 + op2 }),
            HIGH_ORDER,
        );

        let result = calculator.calculate("4+(2");
        assert_eq!(result.is_err(), true);

        let token = result.err().unwrap();
        assert_eq!(token.get_pos(), 2);
        assert_eq!(token.get_value(), "(".to_string());

        let result = calculator.calculate("4+2)");
        assert_eq!(result.is_err(), true);

        let token = result.err().unwrap();
        assert_eq!(token.get_pos(), 3);
        assert_eq!(token.get_value(), ")".to_string());
    }

    #[test]
    fn it_can_operate_multiple_unary() {
        let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
        calculator.add_prefix(
            "-".to_ascii_lowercase(),
            "Negation".to_string(),
            Box::new(|op1| { -op1.clone() }),
            HIGH_ORDER,
        );

        let result = calculator.calculate("---2");
        assert_eq!(result.ok().unwrap(), -2)
    }
}