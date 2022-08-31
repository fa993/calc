#[cfg(test)]
fn evaluate_expr_calc(buffer: &str) -> crate::lib::node::CalcNode {
    use crate::{
        eval,
        lib::{
            context::{ContextManager, ContextType},
            node::CalcNode,
        },
    };

    let mut buffer_part_two = String::new();
    let mut nodes = Vec::<Option<CalcNode>>::new();
    let mut ctx = ContextManager::new();
    ctx.push_stack_frame(ContextType::Calculate);

    let ans = eval(
        &mut buffer.to_string(),
        &mut buffer_part_two,
        &mut nodes,
        &mut ctx,
    );

    return ans;
}

#[test]
#[cfg(test)]
pub fn one() {
    let ans = evaluate_expr_calc("2 + 11 * 4");
    assert_eq!(
        ans,
        crate::lib::node::CalcNode::SingleValue(2.0 + 11.0 * 4.0)
    );
}

#[test]
#[cfg(test)]
pub fn two() {
    let ans = evaluate_expr_calc("(2 + 11) * 4");
    assert_eq!(
        ans,
        crate::lib::node::CalcNode::SingleValue((2.0 + 11.0) * 4.0)
    );
}
