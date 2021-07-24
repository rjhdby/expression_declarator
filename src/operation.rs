use crate::calculator::OperationExecutor;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum OperationType {
    Constant,
    Prefix,
    Postfix,
    Infix,
}

pub struct Operation<T: Clone> {
    pub signature: String,
    pub description: String,
    pub op_type: OperationType,
    pub operands: u8,
    pub priority: u8,
    pub executor: Box<dyn OperationExecutor<T>>,
}

impl<T: 'static + Clone> Clone for Operation<T> {
    fn clone(&self) -> Self {
        return Operation {
            signature: self.signature.clone(),
            description: self.description.clone(),
            op_type: self.op_type.clone(),
            operands: self.operands,
            priority: self.priority,
            executor: self.executor.clone(),
        };
    }
}

impl<T: 'static + Clone> Operation<T> {
    pub fn pretty_print(&self) -> String {
        return match self.op_type.clone() {
            OperationType::Constant => self.signature.clone(),
            OperationType::Prefix => format!("{}(x)", &self.signature),
            OperationType::Postfix => format!("x{}", &self.signature),
            OperationType::Infix => format!("x{}y", &self.signature),
        };
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }

    pub fn priority(&self) -> u8 {
        return self.priority;
    }
}