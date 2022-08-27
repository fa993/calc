use crate::{
    eval,
    lib::{context::Context, funcs::assemble_map_calc, node::CalcNode},
};

#[test]
pub fn one() {
    let mut lkps = assemble_map_calc();
    let mut buffer = "2 + 11 * 4".to_string();
    let mut buffer_part_two = String::new();
    let mut nodes = Vec::<Option<CalcNode>>::new();
    let mut ctx = Context::Calculate;

    let ans = eval(
        &mut buffer,
        &mut buffer_part_two,
        &mut nodes,
        &mut lkps,
        &mut ctx,
    );

    assert_eq!(ans, CalcNode::SingleValue(2.0 + 11.0 * 4.0));
}

#[test]
pub fn two() {
    let mut lkps = assemble_map_calc();
    let mut buffer = "(2 + 11) * 4".to_string();
    let mut buffer_part_two = String::new();
    let mut nodes = Vec::<Option<CalcNode>>::new();
    let mut ctx = Context::Calculate;

    let ans = eval(
        &mut buffer,
        &mut buffer_part_two,
        &mut nodes,
        &mut lkps,
        &mut ctx,
    );

    assert_eq!(ans, CalcNode::SingleValue((2.0 + 11.0) * 4.0));
}
