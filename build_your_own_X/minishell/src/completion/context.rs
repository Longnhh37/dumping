use super::types::CompletionKind;
use crate::parser::lexer::{Token, tokenize};

const SPECIAL_BUILTINS: &[&str] = &["cd", "export", "unset"];

// ------------------------------------------------------------------------
// Public API
// ------------------------------------------------------------------------

pub struct CompletionContext {
    pub kind: CompletionKind,
    pub word_start: usize,
    pub prefix: String,
}

pub fn analyze(line: &str, pos: usize) -> CompletionContext {
    let line = &line[..pos];

    let ctx = try_tokenize_for_completion(line).unwrap_or_else(|| fallback_analyze(line));

    classify(ctx)
}

// ------------------------------------------------------------------------
// Internal result type
// ------------------------------------------------------------------------

struct RawCtx {
    word_start: usize,
    prefix: String,
    is_cmd_pos: bool,
    cmd_name: Option<String>,
    is_redirect_target: bool,
}

fn classify(ctx: RawCtx) -> CompletionContext {
    let RawCtx {
        word_start,
        prefix,
        is_cmd_pos,
        cmd_name,
        is_redirect_target,
    } = ctx;

    // 1. redirect is always file
    if is_redirect_target {
        return CompletionContext {
            kind: CompletionKind::File,
            word_start,
            prefix,
        };
    }

    // 2. $VAR completion
    if prefix.starts_with('$') {
        let env_prefix = prefix
            .trim_start_matches('$')
            .trim_start_matches('{')
            .to_string();

        return CompletionContext {
            kind: CompletionKind::EnvVar,
            word_start,
            prefix: env_prefix,
        };
    }

    // 3. command position
    if is_cmd_pos {
        if prefix.contains('/') {
            return CompletionContext {
                kind: CompletionKind::File,
                word_start,
                prefix,
            };
        }

        return CompletionContext {
            kind: CompletionKind::Command,
            word_start,
            prefix,
        };
    }

    // 4. argument position
    if let Some(cmd) = cmd_name && SPECIAL_BUILTINS.contains(&cmd.as_str()) {
            return CompletionContext {
                kind: CompletionKind::Argument { cmd },
                word_start,
                prefix,
            };
    }

    // 5. Default: file completion
    CompletionContext {
        kind: CompletionKind::File,
        word_start,
        prefix,
    }
}

fn try_tokenize_for_completion(line: &str) -> Option<RawCtx> {
    let tokens = tokenize(line).ok()?;

    let mut groups: Vec<Vec<String>> = vec![vec![]];
    let mut cur_word = String::new();
    let mut skip_next = false;

    for tok in tokens {
        match tok {
            Token::Literal(s) => cur_word.push_str(&s),

            Token::VarExpand(v) => {
                cur_word.push('$');
                cur_word.push_str(&v);
            }

            Token::LastStatus => cur_word.push_str("$?"),

            Token::WordSep => {
                if !cur_word.is_empty() {
                    let w = std::mem::take(&mut cur_word);
                    if !skip_next {
                        groups.last_mut()?.push(w);
                    }
                    skip_next = false;
                }
            }

            Token::Pipe => {
                if !cur_word.is_empty() {
                    let w = std::mem::take(&mut cur_word);
                    if !skip_next {
                        groups.last_mut()?.push(w);
                    }
                    skip_next = false;
                    groups.push(vec![]);
                }
            }

            Token::RedirectOut
            | Token::RedirectIn
            | Token::RedirectAppend
            | Token::RedirectErr
            | Token::RedirectErrAppend => {
                if !cur_word.is_empty() {
                    let w = std::mem::take(&mut cur_word);
                    if !skip_next {
                        groups.last_mut()?.push(w);
                    }
                }
                skip_next = true;
            }
        }
    }

    let current_group = groups.last()?;

    let word_start = word_start_from_raw(line);
    let prefix = cur_word;
    let is_redirect_target = skip_next;
    let is_cmd_pos = current_group.is_empty() && !is_redirect_target;
    let cmd_name = current_group.first().cloned();

    Some(RawCtx {
        word_start,
        prefix,
        is_cmd_pos,
        cmd_name,
        is_redirect_target,
    })
}

fn fallback_analyze(line: &str) -> RawCtx {
    let word_start = word_start_from_raw(line);
    let prefix = line[word_start..].to_string();
    let before = &line[..word_start];

    let is_redirect_target = is_after_redirect(before);
    let cmd_tokens: Vec<&str> = extract_cmd_tokens(before);
    let is_cmd_pos = cmd_tokens.is_empty();
    let cmd_name = cmd_tokens.first().map(|s| s.to_string());

    RawCtx {
        word_start,
        prefix,
        is_cmd_pos,
        cmd_name,
        is_redirect_target,
    }
}

// ------------------------------------------------------------------------
// Helpers
// ------------------------------------------------------------------------

fn word_start_from_raw(line: &str) -> usize {
    line.rfind(|c: char| " \t|&;()<>".contains(c))
        .map(|i| i + 1)
        .unwrap_or(0)
}

fn is_after_redirect(before: &str) -> bool {
    let trimmed = before.trim_end_matches(|c: char| [' ', '\t'].contains(&c));

    trimmed.ends_with('>') || trimmed.ends_with('<')
}

fn extract_cmd_tokens(s: &str) -> Vec<&str> {
    let after_last_pipe = s.rsplit_once('|').map(|(_, r)| r).unwrap_or(s);

    let raw: Vec<&str> = after_last_pipe
        .split(|c: char| " \t;()".contains(c))
        .filter(|t| !t.is_empty())
        .collect();

    let mut result = Vec::new();
    let mut skip_next = false;

    for tok in raw {
        if skip_next {
            skip_next = false;
            continue;
        }

        if matches!(tok, ">" | ">>" | "<" | "2>" | "2>>" | "&>") {
            skip_next = true;
            continue;
        }

        if tok.starts_with('>') || tok.starts_with('<') {
            continue;
        }

        result.push(tok);
    }

    result
}
