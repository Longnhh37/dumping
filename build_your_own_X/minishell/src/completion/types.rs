use rustyline::completion::Candidate;

#[derive(Debug, Clone)]
pub struct CompletionCandidate {
    display: String,
    replacement: String,
}

impl CompletionCandidate {
    pub fn new(display: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self {
            display: display.into(),
            replacement: replacement.into(),
        }
    }
}

impl Candidate for CompletionCandidate {
    fn display(&self) -> &str { &self.display }
    fn replacement(&self) -> &str { &self.replacement }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompletionKind {
    Command,
    Argument { cmd: String },
    EnvVar,
    File,
}
