use super::super::datatype::function::FunctionT;

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Number(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Function {
    Sum(Value),
    Avg(Value),
    Count(Value),
    Max(Value),
    Min(Value),
    Concat(Vec<Value>),
}

impl Function {
    pub fn new(func: FunctionT, args: Vec<Value>) -> Self {
        match func {
            FunctionT::Sum => Self::Sum(args[0].clone()),
            FunctionT::Avg => Self::Avg(args[0].clone()),
            FunctionT::Count => Self::Count(args[0].clone()),
            FunctionT::Max => Self::Max(args[0].clone()),
            FunctionT::Min => Self::Min(args[0].clone()),
            FunctionT::Concat => Self::Concat(args.clone()),
        }
    }
}