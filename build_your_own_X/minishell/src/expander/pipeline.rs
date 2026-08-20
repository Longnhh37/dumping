use crate::expander::command::{expand_command, ExpandedCommand};
use crate::expander::error::ExpandError;
use crate::parser::ast::AstNode;
use crate::runtime::ShellState;

pub fn expand_pipeline(
    node: AstNode,
    state: &ShellState
) -> Result<Vec<ExpandedCommand>, ExpandError> {
    let mut out = Vec::new();

    flatten(node, state, &mut out)?;

    Ok(out)
}

fn flatten(
    node: AstNode,
    state: &ShellState,
    out: &mut Vec<ExpandedCommand>
) -> Result<(), ExpandError> {
    match node {
        AstNode::Command(cmd) => {
            out.push(expand_command(&cmd, state)?);
        }

        AstNode::Pipeline(left, right) => {
            flatten(*left, state, out)?;
            flatten(*right, state, out)?;
        }
    }

    Ok(())
}
