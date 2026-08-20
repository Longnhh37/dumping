use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum CompSpec {
    List(Vec<String>),
    Command(String),
    File,
    Dir,
}

#[derive(Debug)]
pub enum ExecStatus {
    Code(i32),
    Exit(i32),
}

#[derive(Debug, Default)]
pub struct ShellState {
    pub last_status: i32,
    pub comp_registry: HashMap<String, CompSpec>,
    pub vars: HashMap<String, String>,
    pub exported: HashSet<String>,
}

impl ShellState {
    pub fn new() -> Self {
        Self::default()
    }

    /// priority: shell-local vars > fallback to process env vars
    pub fn get_var(&self, name: &str) -> Option<String> {
        self.vars
            .get(name)
            .cloned()
            .or_else(|| std::env::var(name).ok())
    }

    /// set shell-local var
    pub fn set_var(&mut self, name: String, value: String) {
        self.vars.insert(name, value);
    }

    pub fn export_var(&mut self, name: String, value: String) {
        unsafe {
            std::env::set_var(&name, &value);
        }
        self.exported.insert(name.clone());
        self.vars.insert(name, value);
    }

    pub fn unset_var(&mut self, name: &str) {
        self.vars.remove(name);
        self.exported.remove(name);
        unsafe {
            std::env::remove_var(name);
        }
    }
}

pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
