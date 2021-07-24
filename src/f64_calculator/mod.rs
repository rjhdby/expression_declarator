use crate::declarator::{ExpressionDeclarator, HIGHEST_ORDER, LOWEST_ORDER, LOW_ORDER, MEDIUM_ORDER, HIGH_ORDER, PrimitiveHandler};
use regex::Regex;
use std::str::FromStr;
use std::f64::consts::{PI, E};
use lazy_static::lazy_static;

struct F64Handler {}

impl PrimitiveHandler<f64> for F64Handler {
    fn from_string(&self, input: &String) -> Result<f64, ()> {
        let result = f64::from_str(input);
        if result.is_err() {
            return Result::Err(())
        }

        return Result::Ok(result.ok().unwrap());
    }

    fn can_start_with(&self, input: String) -> bool {
        return PRIMITIVE_INCOMPLETE_1.is_match(&input) || PRIMITIVE_INCOMPLETE_2.is_match(&input);
    }
}

pub fn f64_calculator() -> ExpressionDeclarator<f64> {
    let mut calculator = ExpressionDeclarator::<f64>::new(Box::new(F64Handler {}));

    calculator.add_prefix(
        "-".to_string(),
        "Negation".to_string(),
        Box::new(|op1| { -op1.clone() }),
        HIGH_ORDER
    );

    calculator.add_infix(
        "+".to_string(),
        "Addition".to_string(),
        Box::new(|op1, op2| { op1 + op2 }),
        LOWEST_ORDER,
    );
    calculator.add_infix(
        "-".to_string(),
        "Subtraction".to_string(),
        Box::new(|op1, op2| { op1 - op2 }),
        LOWEST_ORDER,
    );
    calculator.add_infix(
        "*".to_string(),
        "Multiplication".to_string(),
        Box::new(|op1, op2| { op1 * op2 }),
        LOW_ORDER,
    );
    calculator.add_infix(
        "/".to_string(),
        "Division".to_string(),
        Box::new(|op1, op2| { op1 / op2 }),
        LOW_ORDER,
    );
    calculator.add_infix(
        "^".to_string(),
        "Product".to_string(),
        Box::new(|op1, op2| { op1.powf(op2) }),
        MEDIUM_ORDER,
    );
    calculator.add_prefix(
        "sqrt".to_string(),
        "Square root".to_string(),
        Box::new(|op1| { op1.sqrt() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "sin".to_string(),
        "Sine".to_string(),
        Box::new(|op1| { op1.sin() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "cos".to_string(),
        "Cosine".to_string(),
        Box::new(|op1| { op1.cos() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "ln".to_string(),
        "Natural logarithm".to_string(),
        Box::new(|op1| { op1.ln() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "log10".to_string(),
        "Common logarithm".to_string(),
        Box::new(|op1| { op1.log10() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "log2".to_string(),
        "Binary logarithm".to_string(),
        Box::new(|op1| { op1.log2() }),
        HIGHEST_ORDER
    );
    calculator.add_prefix(
        "exp".to_string(),
        "Exponent".to_string(),
        Box::new(|op1| { op1.exp() }),
        HIGHEST_ORDER
    );
    calculator.add_constant(
        "pi".to_string(),
        "Constant Pi=3.1415...".to_string(),
        PI
    );
    calculator.add_constant(
        "e".to_string(),
        "Constant e=2.7182....".to_string(),
        E,
    );

    return calculator;
}

lazy_static! {
    static ref PRIMITIVE_INCOMPLETE_1: Regex = Regex::new(r"^(\d+|\d+\.\d*)$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_2: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d*$").unwrap();
}
