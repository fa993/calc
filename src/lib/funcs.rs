use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use crate::{CalcNode, CalcFunctionData};
use crate::lib::EvalFunction;

use super::node::CalcOperatorType;


pub fn bound_check(re: &Vec<CalcNode>, size: usize) {
    if re.len() != size {
        panic!("Incorrect param length");
    }
}

pub fn unary_fn(nm: &str, re: Vec<CalcNode>, to_do: &dyn Fn(f64) -> f64, op: CalcOperatorType, counter: Arc<AtomicUsize>) -> CalcNode {
    if let CalcNode::SingleValue(x) = re[0] {
        return CalcNode::SingleValue(to_do(x));
    }
    return CalcNode::Function(CalcFunctionData{
        name: nm.to_string(),
        params: re,
        operator : Some(op),
        brackets: false,
        id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    });
}

pub fn binary_fn(nm: &str, re: Vec<CalcNode>, to_do: &dyn Fn(f64, f64) -> f64, op: CalcOperatorType, counter: Arc<AtomicUsize>) -> CalcNode {
    binary_fn__(nm, re, to_do, Some(op), counter)
}

pub fn binary_fn__(nm: &str, re: Vec<CalcNode>, to_do: &dyn Fn(f64, f64) -> f64, op: Option<CalcOperatorType>, counter: Arc<AtomicUsize>) -> CalcNode {
    if let CalcNode::SingleValue(x) = re[0] {
        if let CalcNode::SingleValue(y) = re[1] {
            return CalcNode::SingleValue(to_do(x, y));
        }
    }
    return CalcNode::Function(CalcFunctionData{
        name: nm.to_string(),
        params: re,
        operator: op,
        brackets: false,
        id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
    });
}

pub fn assemble_map_calc() -> HashMap<String, EvalFunction> {
    let mut ur: HashMap<String, EvalFunction> = HashMap::new();

    ur.insert("add".to_string(), Box::new(|t, counter| {
        bound_check(&t, 2);
        binary_fn("add", t, &|x, y| x + y, CalcOperatorType::Plus, counter)
    }) as Box<_>);

    ur.insert("negate".to_string(), Box::new(|mut t: Vec<_>,  counter| {
        if t.len() == 1 {
            t.insert(0, CalcNode::SingleValue(0.0));
        }
        bound_check(&t, 2);
        binary_fn("negate", t, &|x, y| x - y, CalcOperatorType::Minus, counter)
    }) as Box<_>);

    ur.insert("multiply".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("multiply", t, &|x, y| x * y, CalcOperatorType::Asterisk, counter)
    }) as Box<_>);

    ur.insert("inverse".to_string(), Box::new(|t, counter| {
        bound_check(&t, 2);
        binary_fn("inverse", t, &|x, y| x / y, CalcOperatorType::Slash, counter)
    }) as Box<_>);

    ur.insert("power".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("power", t, &|x, y| x.powf(y), CalcOperatorType::Caret, counter)
    }) as Box<_>);

    ur.insert("modulus".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("modulus", t, &|x, y| x % y, CalcOperatorType::Modulus, counter)
    }) as Box<_>);

    ur.insert("and".to_string(), Box::new(|t, counter| {
        bound_check(&t, 2);
        binary_fn("and", t, &|x, y| ((x as u64) & (y as u64)) as f64, CalcOperatorType::Ampersand, counter)
    }) as Box<_>);

    ur.insert("or".to_string(), Box::new(|t: Vec<_>,  counter| {
        bound_check(&t, 2);
        binary_fn("or", t, &|x, y| ((x as u64) | (y as u64)) as f64, CalcOperatorType::Pipe, counter)
    }) as Box<_>);

    ur.insert("not".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 1);
        unary_fn("not", t, &|x| (!(x as u64)) as f64, CalcOperatorType::Tild, counter)
    }) as Box<_>);

    ur.insert("xor".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("xor", t, &|x, y| ((x as u64) ^ (y as u64)) as f64, CalcOperatorType::Caret, counter)
    }) as Box<_>);

    return ur;
}

pub fn assemble_map_veri() -> HashMap<String, EvalFunction> {
    let mut ur: HashMap<String, EvalFunction> = HashMap::new();

    ur.insert("multiply".to_string(), Box::new(|t, counter| {
        bound_check(&t, 2);
        binary_fn("and", t, &|x, y| ((x as u64) & (y as u64)) as f64, CalcOperatorType::Ampersand, counter)
    }) as Box<_>);

    ur.insert("and".to_string(), Box::new(|t, counter| {
        bound_check(&t, 2);
        binary_fn("and", t, &|x, y| ((x as u64) & (y as u64)) as f64, CalcOperatorType::Ampersand, counter)
    }) as Box<_>);

    ur.insert("add".to_string(), Box::new(|t: Vec<_>,  counter| {
        bound_check(&t, 2);
        binary_fn("or", t, &|x, y| ((x as u64) | (y as u64)) as f64, CalcOperatorType::Pipe, counter)
    }) as Box<_>);

    ur.insert("or".to_string(), Box::new(|t: Vec<_>, counter| {
        bound_check(&t, 2);
        binary_fn("or", t, &|x, y| ((x as u64) | (y as u64)) as f64, CalcOperatorType::Pipe, counter)
    }) as Box<_>);

    ur.insert("not".to_string(), Box::new(|t, counter| {
        bound_check(&t, 1);
        unary_fn("not", t, &|x| (!(x as u64)) as f64, CalcOperatorType::Tild, counter)
    }) as Box<_>);

    ur.insert("inverse".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("xor", t, &|x, y| ((x as u64) ^ (y as u64)) as f64, CalcOperatorType::Caret, counter)
    }) as Box<_>);

    ur.insert("xor".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn("xor", t, &|x, y| ((x as u64) ^ (y as u64)) as f64, CalcOperatorType::Caret, counter)
    }) as Box<_>);

    ur.insert("nand".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn__("nand", t, &|x, y| !((x as u64) & (y as u64)) as f64, None, counter)
    }) as Box<_>);

    ur.insert("nor".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn__("nor", t, &|x, y| !((x as u64) | (y as u64)) as f64, None, counter)
    }) as Box<_>);

    ur.insert("xnor".to_string(), Box::new(|t,  counter| {
        bound_check(&t, 2);
        binary_fn__("xnor", t, &|x, y| !((x as u64) ^ (y as u64)) as f64, None, counter)
    }) as Box<_>);

    return ur;
}

pub fn assemble_map_veri_nand() -> HashMap<String, EvalFunction> {
    let mut ur: HashMap<String, EvalFunction> = HashMap::new();

    ur.insert("multiply".to_string(), Box::new(|t, counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);
        let first = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets:false,
            operator: None,
            params: t,
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        let first_pt_2 = first.clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![first, first_pt_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
    }) as Box<_>);

    ur.insert("and".to_string(), Box::new(|t, counter: Arc<AtomicUsize>| {
        let first = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets:false,
            operator: None,
            params: t,
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        let first_pt_2 = first.clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![first, first_pt_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
    }) as Box<_>);

    ur.insert("add".to_string(), Box::new(|t: Vec<_>,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);

        let first_param = t[0].clone();
        let first_param_2 = t[0].clone();

        let first_not = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets:false,
            operator: None,
            params: vec![first_param, first_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let second_param = t[1].clone();
        let second_param_2 = t[1].clone();

        let second_not = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![second_param, second_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let third_nand = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![first_not, second_not],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        third_nand

    }) as Box<_>);

    ur.insert("or".to_string(), Box::new(|t: Vec<_>,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);
        let first_param = t[0].clone();
        let first_param_2 = t[0].clone();

        let first_not = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets:false,
            operator: None,
            params: vec![first_param, first_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let second_param = t[1].clone();
        let second_param_2 = t[1].clone();

        let second_not = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![second_param, second_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let third_nand = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![first_not, second_not],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        third_nand
    }) as Box<_>);

    ur.insert("not".to_string(), Box::new(|t,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 1);
        let yy = t[0].clone();
        let yy_pt = t[0].clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nand".to_string(),
            brackets: false,
            operator: None,
            params: vec![yy, yy_pt],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
        // unary_fn("not", t, &|x| (!(x as u64)) as f64, CalcOperatorType::Tild, width, depth)
    }) as Box<_>);

    return ur;
}

pub fn assemble_map_veri_nor() -> HashMap<String, EvalFunction> {
    let mut ur: HashMap<String, EvalFunction> = HashMap::new();

    ur.insert("add".to_string(), Box::new(|t, counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);
        let first = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets:false,
            operator: None,
            params: t,
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        let first_pt_2 = first.clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![first, first_pt_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
    }) as Box<_>);

    ur.insert("or".to_string(), Box::new(|t, counter: Arc<AtomicUsize>| {
        let first = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets:false,
            operator: None,
            params: t,
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        let first_pt_2 = first.clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![first, first_pt_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
    }) as Box<_>);

    ur.insert("multiply".to_string(), Box::new(|t: Vec<_>,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);

        let first_param = t[0].clone();
        let first_param_2 = t[0].clone();

        let first_not = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets:false,
            operator: None,
            params: vec![first_param, first_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let second_param = t[1].clone();
        let second_param_2 = t[1].clone();

        let second_not = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![second_param, second_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let third_nand = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![first_not, second_not],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        third_nand

    }) as Box<_>);

    ur.insert("and".to_string(), Box::new(|t: Vec<_>,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 2);
        let first_param = t[0].clone();
        let first_param_2 = t[0].clone();

        let first_not = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets:false,
            operator: None,
            params: vec![first_param, first_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let second_param = t[1].clone();
        let second_param_2 = t[1].clone();

        let second_not = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![second_param, second_param_2],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

        let third_nand = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![first_not, second_not],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        third_nand
    }) as Box<_>);

    ur.insert("not".to_string(), Box::new(|t,  counter: Arc<AtomicUsize>| {
        bound_check(&t, 1);
        let yy = t[0].clone();
        let yy_pt = t[0].clone();
        let second = CalcNode::Function(CalcFunctionData {
            name: "nor".to_string(),
            brackets: false,
            operator: None,
            params: vec![yy, yy_pt],
            id: counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        second
        // unary_fn("not", t, &|x| (!(x as u64)) as f64, CalcOperatorType::Tild, width, depth)
    }) as Box<_>);

    return ur;
}
