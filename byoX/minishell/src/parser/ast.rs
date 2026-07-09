#[derive(Debug, Clone)]
pub enum AstNode {
    Command(Command),
    Pipeline(Box<AstNode>, Box<AstNode>),
}

#[derive(Debug, Clone)]
pub struct Word {
    pub parts: Vec<WordPart>,
}

impl Word {
    pub fn new(parts: Vec<WordPart>) -> Self {
        Self { parts }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: Word,
    pub args: Vec<Word>,
    pub redirect: Redirect,
}

#[derive(Debug, Clone)]
pub enum Redirect {
    None,
    Input(Word),
    Output(Word),
    Append(Word),
    ErrorOutput(Word),
    ErrorAppend(Word),
}

#[derive(Debug, Clone)]
pub enum WordPart {
    Literal(String),
    Var(String),
    LastStatus,
}
