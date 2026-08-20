use crate::expander::{
    error::ExpandError,
    glob::expand_glob,
    redirect::{ExpandedRedirect, expand_redirect},
    word::expand_word,
};
use crate::parser::ast::Command;
use crate::runtime::ShellState;

#[derive(Debug, Clone)]
pub struct ExpandedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub redirect: ExpandedRedirect,
}

pub fn expand_command(cmd: &Command, state: &ShellState) -> Result<ExpandedCommand, ExpandError> {
    let expanded_name = expand_word(&cmd.name, state);

    let mut expanded_args = Vec::new();

    for arg in &cmd.args {
        let expanded = expand_word(arg, state);
        let globbed = expand_glob(expanded);

        expanded_args.extend(globbed);
    }

    let redirect = expand_redirect(&cmd.redirect, state)?;

    Ok(ExpandedCommand {
        name: expanded_name,
        args: expanded_args,
        redirect,
    })
}
