pub mod context;
pub mod entity;
pub mod funcs;
pub mod node;

use std::sync::{atomic::AtomicUsize, Arc};

use self::node::CalcNode;

pub type EvalFunction = Box<dyn Fn(Vec<CalcNode>, Arc<AtomicUsize>) -> CalcNode>;
