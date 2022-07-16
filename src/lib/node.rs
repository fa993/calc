use std::convert::TryFrom;
use std::fmt;
use std::collections::HashMap;


#[derive(Debug)]
pub enum CalcNodeError {
    OperatorConversionError(String),
    OperatorMethodBindingError(CalcOperatorType),
}

impl fmt::Display for CalcNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcNodeError::OperatorConversionError(x) => {
                write!(f, "{} {}", "No Operator found for character(s):", x)
            }
            CalcNodeError::OperatorMethodBindingError(x) => {
                write!(f, "{} {}", "No method binding for operator: ", x)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CalcFunctionData {
    pub name: String,
    pub params: Vec<CalcNode>,
}

impl fmt::Display for CalcFunctionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut t = self.name.clone();
        self.visit(&mut |f| {
            t.push_str(&f.to_string());
        });
        write!(f, "Function Data of {}", t)
    }
}

impl CalcFunctionData {
    pub fn new(name: &str) -> CalcFunctionData {
        CalcFunctionData {
            name: String::from(name),
            params: vec![],
        }
    }

    fn visit(&self, visitor: &mut dyn FnMut(&CalcNode)) {
        for d in &self.params {
            visitor(&d);
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CalcOperatorType {
    Plus,
    Minus,
    Asterisk,
    Slash,
    ParenthesisOpen,
    ParenthesisClose,
    Caret,
    Modulus,
    Comma,
}

impl CalcOperatorType {
    pub fn get_function_bindings(&self) -> Result<&str, CalcNodeError> {
        match self {
            CalcOperatorType::Plus => Ok("add"),
            CalcOperatorType::Minus => Ok("negate"),
            CalcOperatorType::Asterisk => Ok("multiply"),
            CalcOperatorType::Slash => Ok("inverse"),
            CalcOperatorType::Caret => Ok("power"),
            CalcOperatorType::Modulus => Ok("modulus"),
            _ => Err(CalcNodeError::OperatorMethodBindingError(*self)),
        }
    }
}

impl TryFrom<&str> for CalcOperatorType {
    type Error = CalcNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(CalcOperatorType::Plus),
            "-" => Ok(CalcOperatorType::Minus),
            "*" => Ok(CalcOperatorType::Asterisk),
            "/" => Ok(CalcOperatorType::Slash),
            "(" => Ok(CalcOperatorType::ParenthesisOpen),
            ")" => Ok(CalcOperatorType::ParenthesisClose),
            "^" => Ok(CalcOperatorType::Caret),
            "%" => Ok(CalcOperatorType::Modulus),
            "," => Ok(CalcOperatorType::Comma),
            _ => Err(CalcNodeError::OperatorConversionError(value.to_string())),
        }
    }
}

impl fmt::Display for CalcOperatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CalcOperatorType::Plus => "+",
                CalcOperatorType::Minus => "-",
                CalcOperatorType::Asterisk => "*",
                CalcOperatorType::Slash => "/",
                CalcOperatorType::ParenthesisOpen => "(",
                CalcOperatorType::ParenthesisClose => ")",
                CalcOperatorType::Caret => "^",
                CalcOperatorType::Modulus => "%",
                CalcOperatorType::Comma => ",",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum CalcNode {
    Text(String),
    Operator(CalcOperatorType),
    SingleValue(f64),
    MultipleValue(Box<[f64]>),
    Function(CalcFunctionData),
}

impl CalcNode {

    pub fn eval(&self, lookups: &HashMap<String, Box<dyn Fn(Vec<CalcNode>) -> CalcNode>>) -> CalcNode {
        match self {
            CalcNode::Function(x) => {
                let asd: Vec<CalcNode> = x.params.iter().map(|y| y.eval(lookups)).collect();
                if let Some(t) = lookups.get(x.name.as_str()) {
                    return t(asd);
                } else {
                    panic!("Unexpected")
                }
            },
            CalcNode::SingleValue(x) => {CalcNode::SingleValue(*x)},
            CalcNode::Text(x) => {CalcNode::Text(x.to_string())},
            _ => panic!("Unexpected")
        }
    }

}

impl fmt::Display for CalcNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcNode::Text(t) => write!(f, "Node of type Text : {}", t),
            CalcNode::Operator(op) => write!(f, "Node of type Operator : {}", op),
            CalcNode::Function(dt) => write!(f, "Node of type Function : {}", dt),
            CalcNode::SingleValue(fl) => write!(f, "Node of type SingleValue: {}", fl),
            CalcNode::MultipleValue(fl) => write!(f, "Node of type MultipleValue: {:#?}", fl),
        }
    }
}
