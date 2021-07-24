pub mod declarator;
pub mod calculator;
pub mod ast;
pub mod token;
pub mod operation;
mod tests;

#[cfg(feature = "f64_calculator")]
pub mod f64_calculator;
#[cfg(feature = "bool_calculator")]
pub mod bool_calculator;
