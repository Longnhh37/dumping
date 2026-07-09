use crate::parser::WordPart;
use crate::parser::ast::Word;
use crate::runtime::ShellState;

pub(super) fn expand_word(word: &Word, state: &ShellState) -> String {
    word.parts
        .iter()
        .map(|p| match p {
            WordPart::Literal(s) => s.clone(),
            WordPart::Var(v) => state.get_var(v).unwrap_or_default(),
            WordPart::LastStatus => state.last_status.to_string(),
        })
        .collect()
}
