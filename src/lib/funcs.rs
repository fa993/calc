use std::collections::HashMap;

use crate::{CalcNode, CalcFunctionData};


pub fn bound_check(re: &Vec<CalcNode>, size: usize) {
    if re.len() != size {
        panic!("Incorrect param length");
    }
}

pub fn binary_fn(nm: &str, re: Vec<CalcNode>, to_do: &dyn Fn(f64, f64) -> f64) -> CalcNode {
    if let CalcNode::SingleValue(x) = re[0] {
        if let CalcNode::SingleValue(y) = re[1] {
            return CalcNode::SingleValue(to_do(x, y));
        }
    }
    return CalcNode::Function(CalcFunctionData{
        name: nm.to_string(),
        params: re,
    });
}

pub fn assemble_map() -> HashMap<String, Box<dyn Fn(Vec<CalcNode>) -> CalcNode>> {
    let mut ur = HashMap::new();

    ur.insert("add".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("add", t, &|x, y| x + y)
    }) as Box<_>);

    ur.insert("negate".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("negate", t, &|x, y| x - y)
    }) as Box<_>);

    ur.insert("multiply".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("multiply", t, &|x, y| x * y)
    }) as Box<_>);

    ur.insert("inverse".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("inverse", t, &|x, y| x / y)
    }) as Box<_>);

    ur.insert("power".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("power", t, &|x, y| x + y)
    }) as Box<_>);

    ur.insert("modulus".to_string(), Box::new(|t| {
        bound_check(&t, 2);
        binary_fn("modulus", t, &|x, y| x % y)
    }) as Box<_>);

    return ur;
}
