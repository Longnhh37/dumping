use crate::expander::{error::ExpandError, glob::expand_glob, word::expand_word};
use crate::parser::ast::{Redirect, Word};
use crate::runtime::ShellState;

#[derive(Debug, Clone)]
pub enum ExpandedRedirect {
    None,
    Input(String),          // < file
    Output(String),         // > file
    Append(String),         // >> file
    ErrorOutput(String),    // 2> file
    ErrorAppend(String),    // 2>> file
}

fn expand_redirect_word(word: &Word, state: &ShellState) -> Result<String, ExpandError> {
    let expanded = expand_word(word, state);
    let globbed = expand_glob(expanded);

    match globbed.len() {
        0 => Err(ExpandError::EmptyRedirect),
        1 => Ok(globbed[0].clone()),
        _ => Err(ExpandError::AmbiguousRedirect),
    }
}

pub(super) fn expand_redirect(
    redirect: &Redirect,
    state: &ShellState,
) -> Result<ExpandedRedirect, ExpandError> {
    match redirect {
        Redirect::None => Ok(ExpandedRedirect::None),

        Redirect::Input(word) => Ok(ExpandedRedirect::Input(expand_redirect_word(word, state)?)),

        Redirect::Output(word) => Ok(ExpandedRedirect::Output(expand_redirect_word(word, state)?)),

        Redirect::Append(word) => Ok(ExpandedRedirect::Append(expand_redirect_word(word, state)?)),

        Redirect::ErrorOutput(word) => Ok(ExpandedRedirect::ErrorOutput(expand_redirect_word(word, state)?)),

        Redirect::ErrorAppend(word) => Ok(ExpandedRedirect::ErrorAppend(expand_redirect_word(word, state)?)),

    }
}
