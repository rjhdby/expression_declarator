# Customisable expression calculator
1. Define your domain (e.g. float algebra, boolean algebra, trigonometry, etc)
2. Define possible operations (e.g. "+", "sin(x)", "|"(OR))
3. Calculate expression `calculator.calculate("-10*sin(23)^4+(1-2/3)")` 

# How it works
1. Tokenize input string according defined domain and operations
2. Build AST (reverse Polish notation method used) 
3. Calculate

# Example 
### Quick example
```rust
struct IntHandler {}

impl PrimitiveHandler<i32> for IntHandler {
    fn from_string(&self, input: &String) -> Result<i32, ()> {
        return Result::Ok(input.parse::<i32>().unwrap())
    }

    fn can_start_with(&self, input: String) -> bool {
        input.chars().all(|it| { it.is_numeric() })
    }
}
fn main() {
    let mut calculator = ExpressionDeclarator::<i32>::new(Box::new(IntHandler {}));
    calculator.add_infix(
        "+".to_string(),
        "Add".to_string(),
        Box::new(|op1, op2| { op1 + op2 }),
        LOW_ORDER,
    );
    calculator.add_infix(
        "/".to_string(),
        "Divide".to_string(),
        Box::new(|op1, op2| { op1 / op2 }),
        HIGH_ORDER,
    );
    
    assert_eq!(calculator.calculate("4+6/2").ok().unwrap(), 7);
    assert_eq!(calculator.calculate("(4+6)/2").ok().unwrap(), 5);
}
```
### Boolean algebra 
See https://github.com/rjhdby/expression_declarator/src/bool_calculator
### f64 calculator
See https://github.com/rjhdby/expression_declarator/src/f64_calculator

# Operation types
```rust
pub enum OperationType {
    Constant,
    Infix,
    Prefix,
    Postfix,
}
```

| Type     | Arguments | Description                                                         | Example                 |
|----------|-----------|---------------------------------------------------------------------|-------------------------|
| Constant | 0         | Predefined primitive value                                          | `Pi=3.14`               |
| Infix    | 2         | Function with two arguments, one on the left and other on the right | `1+2`                   |
| Prefix   | 1         | Function with one argument on the right                             | `-2`, `!true`, `sin(x)` |
| Postfix  | 1         | Function with one argument on the left                              | `2!` - factorial        |

# Priority of operations
Provided as `u8` integer. Higher value assume higher priority.
Priority can be managed with parenthesis. e.g. `10 * 2 + 1 = 21`, but `10 * (2 + 1) = 30` 

### Predefined priorities.
```rust
pub const LOWEST_ORDER: u8 = 10;
pub const LOW_ORDER: u8 = 20;
pub const MEDIUM_ORDER: u8 = 30;
pub const HIGH_ORDER: u8 = 40;
pub const HIGHEST_ORDER: u8 = 50;
pub const ULTIMATE_ORDER: u8 = u8::MAX;
```

# Primitives
Primitives are base entities of domain. 
For math, it can be Integers, Floats, etc. 
For boolean algebra there are two values `true` and `false`, or `1` and `0` as you wish. 

Parser must have possibility to determine primitives inside input string. So, you must realize Handler with `PrimitiveHandler<T>` trait implementation.
```rust
pub trait PrimitiveHandler<T> {
    fn from_string(&self, input: &String) -> Result<T, ()>;
    fn can_start_with(&self, input: String) -> bool;
}
```

`from_string` - must provide primitive from string representation
`can_start_with` - determine if a string can be primitive or not

For example, `float` primitive may have different representations:

- 23.9
- 11 (without dot)
- 0.1e7 (scientific notation)
- 7e-5 (also scientific notation)

And this is `PrimitiveHandler<f64>` realization
```rust
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

lazy_static! {
    static ref PRIMITIVE_INCOMPLETE_1: Regex = Regex::new(r"^(\d+|\d+\.\d*)$").unwrap();
    static ref PRIMITIVE_INCOMPLETE_2: Regex = Regex::new(r"^(\d+|\d+\.\d+)[eE][-+]?\d*$").unwrap();
}
```

# Structs

`ExpressionDeclarator` struct
```rust
struct ExpressionDeclarator<T: Clone>;
```
Functions
```rust
// Constructor
pub fn new(handler: Box<dyn PrimitiveHandler<T>>) -> ExpressionDeclarator<T>;
// General purpose function for creation an operation
pub fn add(
    signature: String,                       // Operation signature, e.g. "+" or "sin"
    description: String,                     // Description
    op_type: OperationType,                  // Operation type
    executor: Box<dyn OperationExecutor<T>>, // Operation implementation
    operands: u8,                            // Number of operands
    order: u8,                               // Priority
);
// Helper function for prefix operations
pub fn add_prefix(
    signature: String,
    description: String,
    executor: Box<dyn UnaryOperationExecutor<T>>,
    order: u8,
);
// Helper function for postfix operations
pub fn add_postfix(
    signature: String,
    description: String,
    executor: Box<dyn UnaryOperationExecutor<T>>,
    order: u8,
);
// Helper function for infix operations
pub fn add_infix(
    signature: String, 
    description: String, 
    executor: Box<dyn BinaryOperationExecutor<T>>, 
    order: u8
);
// Helper function for constants with ULTIMATE_ORDER priority
pub fn add_constant(
    signature: String, 
    description: String, 
    value: T
);

// Calculate expression
pub fn calculate(input: &str) -> Result<T, Token<T>>;
// Tokenize expression
pub fn tokenize(input: &str) -> Result<Vec<Token<T>>, Token<T>>;
// Build AST from input string
pub fn build_ast(input: &str) -> Result<AstNode<T>, Token<T>>;
// Build AST from tokens
pub fn build_ast_from_tokens(tokens: &Vec<Token<T>>) -> Result<AstNode<T>, Token<T>>;
```
`Token` enum

```rust
pub enum Token<T: Clone> {
    WhiteSpace { pos: usize, val: String },
    Open { pos: usize },
    Close { pos: usize },
    Primitive { pos: usize, val: T, original: String },
    Operation { pos: usize, val: Box<Operation<T>> },
    Unknown { pos: usize, val: String },
}
```
Functions
```rust
// Starting position inside string
pub fn get_pos() -> usize;
// String value
pub fn get_value() -> String;
// Return pretty string for token
// e.g. "'sin' at position 5"
pub fn pretty_print() -> String;
```

`AstNode` enum
```rust
pub enum AstNode<T: Clone> {
    Primitive { val: T, token: Token<T> },
    Unary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, token: Token<T> },
    Binary { op: Box<Operation<T>>, p1: Box<AstNode<T>>, p2: Box<AstNode<T>>, token: Token<T> },
}
```
Functions
```rust
// Recursively calculate AST
pub fn calculate() -> Result<T, Token<T>>;
```

`Operation` struct

```rust
pub struct Operation<T: Clone> {
    pub signature: String,
    pub description: String,
    pub op_type: OperationType,
    pub operands: u8,
    pub priority: u8,
    pub executor: Box<dyn OperationExecutor<T>>,
}
```

Functions
```rust
// Return pretty string for operation
pub fn pretty_print() -> String;
```