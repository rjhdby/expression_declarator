use crate::declarator::{ExpressionDeclarator, HIGH_ORDER, LOW_ORDER, PrimitiveHandler};

struct BoolHandler {}

impl PrimitiveHandler<bool> for BoolHandler {
    fn from_string(&self, input: &String) -> Result<bool, ()> {
        return match input.to_lowercase().as_str() {
            "true" => Result::Ok(true),
            "false" => Result::Ok(false),
            _ => Result::Err(()),
        };
    }

    fn can_start_with(&self, input: String) -> bool {
        return "true".starts_with(&input) || "false".starts_with(&input);
    }
}

pub fn boolean_calculator() -> ExpressionDeclarator<bool> {
    let mut calculator = ExpressionDeclarator::<bool>::new(Box::new(BoolHandler {}));

    calculator.add_infix(
        "|".to_string(),
        "OR".to_string(),
        Box::new(|op1, op2| { op1 | op2 }),
        LOW_ORDER,
    );

    calculator.add_infix(
        "&".to_string(),
        "AND".to_string(),
        Box::new(|op1, op2| { op1 & op2 }),
        LOW_ORDER,
    );

    calculator.add_infix(
        "^".to_string(),
        "XOR".to_string(),
        Box::new(|op1, op2| { op1 ^ op2 }),
        LOW_ORDER,
    );

    calculator.add_prefix(
        "!".to_string(),
        "NOT".to_string(),
        Box::new(|op1| { !op1 }),
        HIGH_ORDER,
    );

    return calculator;
}