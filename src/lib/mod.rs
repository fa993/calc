pub mod entity;
pub mod node;
pub mod funcs;
pub mod context;

use std::sync::{atomic::AtomicUsize, Arc};

use self::node::CalcNode;

type EvalFunction = Box<dyn Fn(Vec<CalcNode>, Arc<AtomicUsize>) -> CalcNode>;
