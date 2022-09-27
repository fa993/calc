use lib::node::CalcUserFunctionData;
use std::convert::TryFrom;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

mod lib;
mod test;

use lib::context::*;
use lib::node::{CalcFunctionData, CalcNode, CalcOperatorType};

fn parse_buffer(buffer: &str, next_operator: Option<CalcOperatorType>) -> CalcNode {
    let entity = buffer.parse::<f64>();
    if let Ok(x) = entity {
        return CalcNode::SingleValue(x);
    }
    if let Some(CalcOperatorType::ParenthesisOpen) = next_operator {
        return CalcNode::Function(CalcFunctionData::new(buffer));
    }
    return CalcNode::Text(String::from(buffer));
}

fn find_open_bracket(slice: &[Option<CalcNode>]) -> Option<usize> {
    for i in 0..slice.len() {
        if Some(CalcNode::Operator(CalcOperatorType::ParenthesisOpen)) == slice[i] {
            return Some(i);
        }
    }
    return None;
}

fn find_close_bracket(slice: &[Option<CalcNode>]) -> Option<usize> {
    for i in 0..slice.len() {
        if Some(CalcNode::Operator(CalcOperatorType::ParenthesisClose)) == slice[i] {
            return Some(i);
        }
    }
    return None;
}

fn find_first_ele_in_direction<T>(
    slice: &mut [Option<T>],
    start: usize,
    forward: bool,
) -> Option<usize> {
    return find_ele_in_direction(slice, start, forward, 1);
}

fn find_ele_in_direction<T>(
    slice: &mut [Option<T>],
    start: usize,
    forward: bool,
    occurence: usize,
) -> Option<usize> {
    let mut seen = 0;
    if forward {
        for i in start + 1..slice.len() {
            let t = &slice[i];
            if t.is_some() {
                seen += 1;
                if seen == occurence {
                    return Some(i);
                }
            }
        }
    } else {
        for i in (0..start).rev() {
            let t = &slice[i];
            if t.is_some() {
                seen += 1;
                if seen == occurence {
                    return Some(i);
                }
            }
        }
    }
    return None;
}

fn find_fn_in_direction(
    slice: &mut [Option<CalcNode>],
    start: usize,
    forward: bool,
    occurence: usize,
) -> Option<usize> {
    let mut seen = 0;
    if forward {
        for i in start + 1..slice.len() {
            let t = &slice[i];
            if let Some(CalcNode::Function(_)) = t {
                seen += 1;
                if seen == occurence {
                    return Some(i);
                }
            }
        }
    } else {
        for i in (0..start).rev() {
            let t = &slice[i];
            if let Some(CalcNode::Function(_)) = t {
                seen += 1;
                if seen == occurence {
                    return Some(i);
                }
            }
        }
    }
    return None;
}

fn find_first_fn_in_direction(
    slice: &mut [Option<CalcNode>],
    start: usize,
    forward: bool,
) -> Option<usize> {
    return find_fn_in_direction(slice, start, forward, 1);
}

fn apply_equals(slice: &mut [Option<CalcNode>]) {
    for i in 0..slice.len() {
        if let Some(CalcNode::Operator(CalcOperatorType::Equals)) = slice[i] {
            let body = find_first_ele_in_direction(slice, i, true);
            let def = find_first_ele_in_direction(slice, i, false);

            if body.is_some() && def.is_some() {
                let sd = body.expect("not possible");
                let sf = def.expect("not possible");
                let res = match slice[sf].as_ref().expect("not possible") {
                    CalcNode::Text(x) => CalcNode::UserFunction(CalcUserFunctionData {
                        name: x.to_string(),
                        id: 0,
                        params: Vec::new(),
                        eval_tree: Box::new(slice[sd].take().unwrap()),
                    }),
                    CalcNode::Function(x) => CalcNode::UserFunction(CalcUserFunctionData {
                        name: x.name.to_string(),
                        id: 0,
                        params: x.params.iter().map(|f| f.to_string()).collect(),
                        eval_tree: Box::new(slice[sd].take().unwrap()),
                    }),
                    _ => {
                        panic!("Mismatched ")
                    }
                };
                slice[sf] = Some(res);
                slice[i].take();
            }
        }
    }
}

fn apply_precedence_unary(slice: &mut [Option<CalcNode>], operator: CalcOperatorType) {
    let name = operator.get_function_bindings().unwrap();
    for i in 0..slice.len() {
        let t = &slice[i];
        match t {
            Some(CalcNode::Operator(op)) if *op == operator => {
                let mut y = CalcFunctionData::new(name);

                let item_index = find_first_ele_in_direction(slice, i, true);

                if let Some(x) = item_index {
                    let item = (&mut slice[x]).take().unwrap();
                    y.push_param(item);
                }
                slice[i] = Some(CalcNode::Function(y));
            }
            _ => {}
        }
    }
}

fn apply_precedence_binary(slice: &mut [Option<CalcNode>], operator: CalcOperatorType) {
    let name = operator.get_function_bindings().unwrap();
    for i in 0..slice.len() {
        let t = &slice[i];
        match t {
            Some(CalcNode::Operator(op)) if *op == operator => {
                let mut y = CalcFunctionData::new(name);

                let item_index = find_first_ele_in_direction(slice, i, false);

                if let Some(x) = item_index {
                    let item = (&mut slice[x]).take().unwrap();
                    y.push_param(item);
                }

                let item_index = find_first_ele_in_direction(slice, i, true);

                if let Some(x) = item_index {
                    let item = (&mut slice[x]).take().unwrap();
                    y.push_param(item);
                }
                slice[i] = Some(CalcNode::Function(y));
            }
            _ => {}
        }
    }
}

fn apply_precedence_overall(slice: &mut [Option<CalcNode>]) -> usize {
    //first check for bracket

    //first resolve assignments

    //now check for function args
    //now the func should be in the form of #, #, #,

    while let Some(br) = find_open_bracket(slice) {
        let possible_func = find_first_fn_in_direction(slice, br, false);

        let bracket_close = apply_precedence_overall(&mut slice[br + 1..]) + br + 1;
        let mut new_params = vec![];

        if possible_func.is_some() {
            let mut last_ele_was_comma = true;
            for i in (br + 1)..bracket_close {
                let ele = &mut slice[i];
                if ele.is_some() {
                    if last_ele_was_comma {
                        new_params.push(ele.take().unwrap());
                    } else if !last_ele_was_comma
                        && ele.take().unwrap() != CalcNode::Operator(CalcOperatorType::Comma)
                    {
                        panic!("Incorrect function definition");
                    }
                    last_ele_was_comma = !last_ele_was_comma;
                }
            }
            let func_name = slice[possible_func.unwrap()].as_mut();
            if let Some(CalcNode::Function(y)) = func_name {
                y.params = new_params;
            }
        }
        slice[br].take();
        if bracket_close < slice.len() {
            slice[bracket_close].take();
        }
    }

    let bracket_index = find_close_bracket(slice).unwrap_or(slice.len());

    apply_precedence_unary(&mut slice[..bracket_index], CalcOperatorType::Tild);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Caret);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Slash);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Asterisk);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Modulus);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Minus);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Plus);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Ampersand);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Pipe);

    apply_equals(&mut slice[..bracket_index]);

    return bracket_index;
}

fn eval(
    buffer: &mut String,
    buffer_part_two: &mut String,
    nodes: &mut Vec<Option<CalcNode>>,
    ctx: &mut ContextManager,
) -> CalcNode {
    if buffer.starts_with("context") {
        //treat as context command
        // let ctx: Context = buffer["context ".len()..].try_into().expect("No associated context found");
        let mode = &buffer["context ".len()..buffer.len() - 1];
        let c_type: ContextType = mode.try_into().expect("No Associated Context found");
        ctx.push_stack_frame(c_type);
        println!("Parse Complete");
        return CalcNode::NoValue;
    }

    let x = UnicodeSegmentation::graphemes(&buffer[..], true).collect::<Vec<&str>>();

    for y in x {
        let possible_op = CalcOperatorType::try_from(y);
        if possible_op.is_ok() {
            //look at buffer now
            let opera = possible_op.unwrap();
            if !buffer_part_two.is_empty() {
                nodes.push(Some(parse_buffer(&buffer_part_two, Some(opera))));
                buffer_part_two.clear();
            }
            //then pass the operator
            nodes.push(Some(CalcNode::Operator(opera)));
        } else if !y.trim().is_empty() {
            buffer_part_two.push_str(y.trim());
        }
    }
    if !buffer_part_two.is_empty() {
        nodes.push(Some(parse_buffer(&buffer_part_two, None)));
        buffer_part_two.clear();
    }
    apply_precedence_overall(nodes);
    // apply_precedence_rules(&mut nodes, CalcOperatorType::Plus, CalcOperatorType::Minus, "add", "minus");

    println!("{:?}", nodes);

    for t in nodes {
        if let Some(i) = t {
            println!("Parse Complete");
            return ctx.eval(i);
        }
    }
    return CalcNode::NoValue;
}

fn main() {
    let mut buffer = String::new();
    let mut buffer_part_two = String::new();
    let mut nodes = Vec::<Option<CalcNode>>::new();
    let mut ctxs = ContextManager::new();
    ctxs.push_stack_frame(ContextType::Calculate);
    loop {
        io::stdin()
            .read_line(&mut buffer)
            .expect("Something went wrong");

        let ans = eval(&mut buffer, &mut buffer_part_two, &mut nodes, &mut ctxs);

        ctxs.print_result(&ans);

        buffer.clear();
        buffer_part_two.clear();
        nodes.clear();
    }
}
