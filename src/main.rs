use std::convert::TryFrom;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

mod lib;

use lib::node::{CalcFunctionData, CalcNode, CalcOperatorType};
use lib::funcs::*;

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
                    y.params.push(item);
                }

                let item_index = find_first_ele_in_direction(slice, i, true);

                if let Some(x) = item_index {
                    let item = (&mut slice[x]).take().unwrap();
                    y.params.push(item);
                }
                slice[i] = Some(CalcNode::Function(y));
            }
            _ => {}
        }
    }
}

fn apply_precedence_overall(slice: &mut [Option<CalcNode>]) -> usize {
    //first check for bracket
    let bracket_index = find_open_bracket(slice);

    //first resolve assignments

    //now check for function args
    //now the func should be in the form of #, #, #,

    if let Some(br) = bracket_index {
        let possible_func = find_first_ele_in_direction(slice, br, false);

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

    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Slash);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Asterisk);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Minus);
    apply_precedence_binary(&mut slice[..bracket_index], CalcOperatorType::Plus);

    return bracket_index;
}

fn main() {
    let lkps = assemble_map();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Something went wrong");
    let x = UnicodeSegmentation::graphemes(&buffer[..], true).collect::<Vec<&str>>();
    let mut buffer_part_two = String::new();
    let mut nodes = Vec::<Option<CalcNode>>::new();
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
    println!("{:?}", nodes);
    apply_precedence_overall(&mut nodes);
    // apply_precedence_rules(&mut nodes, CalcOperatorType::Plus, CalcOperatorType::Minus, "add", "minus");
    println!("{:?}", nodes);

    for t in nodes {
        if let Some(i) = t {
            println!("{:?}", i.eval(&lkps));
            break;
        }
    }

    println!("Parse Complete");
}
