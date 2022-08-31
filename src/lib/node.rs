use std::convert::TryFrom;
use std::fmt;

use anyhow::anyhow;

#[derive(Debug)]
pub enum CalcNodeError {
    OperatorConversionError(String),
    OperatorMethodBindingError(CalcOperatorType),
}

impl std::error::Error for CalcNodeError {}

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

#[derive(Debug, PartialEq, Clone)]
pub struct CalcFunctionData {
    pub name: String,
    pub params: Vec<CalcNode>,
    pub operator: Option<CalcOperatorType>,
    pub brackets: bool,
    pub id: usize,
}

impl fmt::Display for CalcFunctionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() && self.operator.is_some() {
            if self.brackets {
                "(".fmt(f)?;
            }
            self.visit(&mut |fre, is_last| {
                let opr = self.operator.ok_or_else(|| anyhow!("No Operator"))?;
                if !opr.is_postfix()? {
                    opr.fmt(f)?;
                    fre.fmt(f)?;
                } else if !is_last {
                    fre.fmt(f)?;
                    opr.fmt(f)?;
                } else {
                    fre.fmt(f)?;
                }
                Ok(())
            })
            .map_err(|_| fmt::Error {})?;
            if self.brackets {
                ")".fmt(f)?;
            };
            Ok(())
        } else {
            write!(f, "{}(", self.name).expect("Couldn't display");
            self.visit(&mut |fre, is_last| {
                fre.fmt(f)?;
                if !is_last {
                    f.write_str(", ")?;
                }
                Ok(())
            })
            .map_err(|_| fmt::Error {})?;
            f.write_str(")")
        }
    }
}

impl CalcFunctionData {
    pub fn new(name: &str) -> CalcFunctionData {
        CalcFunctionData {
            name: String::from(name),
            params: vec![],
            operator: None,
            brackets: false,
            id: 0,
        }
    }

    fn visit(
        &self,
        visitor: &mut dyn FnMut(&CalcNode, bool) -> Result<(), anyhow::Error>,
    ) -> Result<(), anyhow::Error> {
        for i in 0..&self.params.len() - 1 {
            visitor(&self.params[i], false)?;
        }
        visitor(&self.params[self.params.len() - 1], true)
    }

    pub fn push_param(&mut self, node: CalcNode) {
        self.params.push(node);
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
    Ampersand,
    Pipe,
    Tild,
    Equals,
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
            CalcOperatorType::Ampersand => Ok("and"),
            CalcOperatorType::Pipe => Ok("or"),
            CalcOperatorType::Tild => Ok("not"),
            _ => Err(CalcNodeError::OperatorMethodBindingError(*self)),
        }
    }

    pub fn is_postfix(&self) -> Result<bool, CalcNodeError> {
        match self {
            CalcOperatorType::Plus
            | CalcOperatorType::Minus
            | CalcOperatorType::Asterisk
            | CalcOperatorType::Slash
            | CalcOperatorType::Caret
            | CalcOperatorType::Modulus
            | CalcOperatorType::Ampersand
            | CalcOperatorType::Pipe => Ok(true),
            CalcOperatorType::Tild => Ok(false),
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
            "&" => Ok(CalcOperatorType::Ampersand),
            "|" => Ok(CalcOperatorType::Pipe),
            "~" => Ok(CalcOperatorType::Tild),
            "=" => Ok(CalcOperatorType::Equals),
            _ => Err(CalcNodeError::OperatorConversionError(value.to_string())),
        }
    }
}

impl fmt::Display for CalcOperatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " {} ",
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
                CalcOperatorType::Ampersand => "&",
                CalcOperatorType::Pipe => "|",
                CalcOperatorType::Tild => "~",
                CalcOperatorType::Equals => "=",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CalcUserFunctionData {
    pub name: String,
    pub id: usize,
    pub params: Vec<String>,
    pub eval_tree: Box<CalcNode>,
}

impl fmt::Display for CalcUserFunctionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CalcNode {
    Text(String),
    Operator(CalcOperatorType),
    SingleValue(f64),
    MultipleValue(Box<[f64]>),
    Function(CalcFunctionData),
    UserFunction(CalcUserFunctionData),
    NoValue,
}

impl fmt::Display for CalcNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcNode::Text(t) => t.fmt(f),
            CalcNode::Operator(op) => write!(f, " {} ", op),
            CalcNode::Function(dt) => dt.fmt(f),
            CalcNode::SingleValue(fl) => fl.fmt(f),
            CalcNode::MultipleValue(fl) => write!(f, "Node of type MultipleValue: {:?}", fl),
            CalcNode::UserFunction(dtu) => dtu.fmt(f),
            CalcNode::NoValue => write!(f, "Node of type NoValue"),
        }
    }
}
