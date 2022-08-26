use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{self, Write};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use crate::lib::EvalFunction;

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
                "(".fmt(f).expect("Couldn't display");
            }
            self.visit(&mut |fre, is_last| {
                fre.fmt(f).expect("Couldn't display");
                if !is_last {
                    self.operator
                        .expect("No operator")
                        .fmt(f)
                        .expect("Operator couldn't be serialized");
                }
            });
            if self.brackets {
                ")".fmt(f).expect("Couldn't display");
            }
            Ok(())
        } else {
            write!(f, "{}(", self.name).expect("Couldn't display");
            self.visit(&mut |fre, is_last| {
                fre.fmt(f).expect("Couldn't display");
                if !is_last {
                    f.write_str(", ").expect("Couldn't display");
                }
            });
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

    fn visit(&self, visitor: &mut dyn FnMut(&CalcNode, bool)) {
        for i in 0..&self.params.len() - 1 {
            visitor(&self.params[i], false);
        }
        visitor(&self.params[self.params.len() - 1], true);
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
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CalcNode {
    Text(String),
    Operator(CalcOperatorType),
    SingleValue(f64),
    MultipleValue(Box<[f64]>),
    Function(CalcFunctionData),
    NoValue,
}

impl CalcNode {
    pub fn eval(&self, lookups: &HashMap<String, EvalFunction>) -> CalcNode {
        self.eval_internal(lookups, Arc::new(AtomicUsize::new(0)))
    }

    fn eval_internal(
        &self,
        lookups: &HashMap<String, EvalFunction>,
        counter: Arc<AtomicUsize>,
    ) -> CalcNode {
        match self {
            CalcNode::Function(x) => {
                let asd: Vec<CalcNode> = x
                    .params
                    .iter()
                    .map(|y| y.eval_internal(lookups, counter.clone()))
                    .collect();
                if let Some(t) = lookups.get(x.name.as_str()) {
                    return t(asd, counter);
                } else {
                    panic!("Unexpected")
                }
            }
            CalcNode::SingleValue(x) => CalcNode::SingleValue(*x),
            CalcNode::Text(x) => CalcNode::Text(x.to_string()),
            _ => panic!("Unexpected"),
        }
    }

    pub fn to_verilog(&self) -> String {
        let mut t = String::new();
        let mut seen = HashSet::new();
        self.to_verilog__(&mut t, &mut seen);

        print!("wire ");
        for t in seen {
            print!("w_{}, ", t);
        }
        println!("");

        return t;
    }

    fn to_verilog__(&self, sout: &mut String, seen: &mut HashSet<usize>) {
        match self {
            CalcNode::Function(x) => {
                x.params.iter().for_each(|y| y.to_verilog__(sout, seen));

                if !seen.insert(x.id) {
                    return;
                }

                if x.params.len() == 2 {
                    write!(
                        sout,
                        "{}({}, {}, {});\n",
                        x.name,
                        format!("w_{}", x.id),
                        if let CalcNode::Text(y) = &x.params[0] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[0] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        },
                        if let CalcNode::Text(y) = &x.params[1] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[1] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        }
                    )
                    .expect("msg");
                } else if x.params.len() == 1 {
                    write!(
                        sout,
                        "{}({}, {});\n",
                        x.name,
                        format!("w_{}", x.id),
                        if let CalcNode::Text(y) = &x.params[0] {
                            format!("{}", y)
                        } else if let CalcNode::Function(y) = &x.params[0] {
                            format!("w_{}", y.id)
                        } else {
                            panic!("Type not allowed")
                        }
                    )
                    .expect("msg");
                }
            }
            _ => {}
        };
    }
}

impl fmt::Display for CalcNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcNode::Text(t) => t.fmt(f),
            CalcNode::Operator(op) => write!(f, " {} ", op),
            CalcNode::Function(dt) => dt.fmt(f),
            CalcNode::SingleValue(fl) => fl.fmt(f),
            CalcNode::MultipleValue(fl) => write!(f, "Node of type MultipleValue: {:?}", fl),
            CalcNode::NoValue => write!(f, "Node of type NoValue"),
        }
    }
}
