use std::fs::{File, OpenOptions};
use std::os::fd::OwnedFd;
use std::process::Stdio;

use crate::expander::redirect::ExpandedRedirect;

// ══════════════════════════════════════════════════════════════════════════════
// StdioSet  —  for std::process::Command (external commands)
// ══════════════════════════════════════════════════════════════════════════════

/// All three stdio streams packaged for `std::process::Command`.
pub struct StdioSet {
    pub stdin: Stdio,
    pub stdout: Stdio,
    pub stderr: Stdio,
}

/// Build a `StdioSet` from the command's redirect spec.
///
/// Unaffected streams default to `Stdio::inherit()`.
/// The pipeline layer is responsible for overriding `stdin`/`stdout` when
/// connecting pipe segments; this function only handles file redirects.
pub fn make_stdio_set(redirect: &ExpandedRedirect) -> StdioSet {
    match redirect {
        ExpandedRedirect::None => StdioSet {
            stdin: Stdio::inherit(),
            stdout: Stdio::inherit(),
            stderr: Stdio::inherit(),
        },

        ExpandedRedirect::Input(path) => StdioSet {
            stdin: open_read(path),
            stdout: Stdio::inherit(),
            stderr: Stdio::inherit(),
        },

        ExpandedRedirect::Output(path) => StdioSet {
            stdin: Stdio::inherit(),
            stdout: open_write(path),
            stderr: Stdio::inherit(),
        },

        ExpandedRedirect::Append(path) => StdioSet {
            stdin: Stdio::inherit(),
            stdout: open_append(path),
            stderr: Stdio::inherit(),
        },

        ExpandedRedirect::ErrorOutput(path) => StdioSet {
            stdin: Stdio::inherit(),
            stdout: Stdio::inherit(),
            stderr: open_write(path),
        },

        ExpandedRedirect::ErrorAppend(path) => StdioSet {
            stdin: Stdio::inherit(),
            stdout: Stdio::inherit(),
            stderr: open_append(path),
        },
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// RedirectFds  —  for fork() + dup2() (builtins inside a pipeline)
// ══════════════════════════════════════════════════════════════════════════════

/// Raw file descriptors for the fork+dup2 code path.
///
/// `None` means "do not dup2 this stream" (keep whatever the child inherits
/// from the pipe plumbing already set up by the pipeline driver).
pub struct RedirectFds {
    pub stdin: Option<OwnedFd>,
    pub stdout: Option<OwnedFd>,
    pub stderr: Option<OwnedFd>,
}

/// Open any file-backed redirects as `OwnedFd`s.
///
/// Call this before `fork()` so errors are surfaced in the parent and the
/// fds are open in both parent and child. The child should drop the parent's
/// copy after dup2'ing.
pub fn open_redirect_fds(redirect: &ExpandedRedirect) -> RedirectFds {
    match redirect {
        ExpandedRedirect::None => RedirectFds {
            stdin: None,
            stdout: None,
            stderr: None,
        },

        ExpandedRedirect::Input(path) => RedirectFds {
            stdin: File::open(path)
                .map_err(|e| eprintln!("redirect <: {path}: {e}"))
                .ok()
                .map(OwnedFd::from),
            stdout: None,
            stderr: None,
        },

        ExpandedRedirect::Output(path) => RedirectFds {
            stdin: None,
            stdout: File::create(path)
                .map_err(|e| eprintln!("redirect >: {path}: {e}"))
                .ok()
                .map(OwnedFd::from),
            stderr: None,
        },

        ExpandedRedirect::Append(path) => RedirectFds {
            stdin: None,
            stdout: OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|e| eprintln!("redirect >>: {path}: {e}"))
                .ok()
                .map(OwnedFd::from),
            stderr: None,
        },

        ExpandedRedirect::ErrorOutput(path) => RedirectFds {
            stdin: None,
            stdout: None,
            stderr: File::create(path)
                .map_err(|e| eprintln!("redirect 2>: {path}: {e}"))
                .ok()
                .map(OwnedFd::from),
        },

        ExpandedRedirect::ErrorAppend(path) => RedirectFds {
            stdin: None,
            stdout: None,
            stderr: OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .map_err(|e| eprintln!("redirect 2>>: {path}: {e}"))
                .ok()
                .map(OwnedFd::from),
        },
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Internal helpers
// ══════════════════════════════════════════════════════════════════════════════

fn open_read(path: &str) -> Stdio {
    File::open(path).map(Stdio::from).unwrap_or_else(|e| {
        eprintln!("redirect <: {path}: {e}");
        Stdio::inherit()
    })
}

fn open_write(path: &str) -> Stdio {
    File::create(path).map(Stdio::from).unwrap_or_else(|e| {
        eprintln!("redirect >: {path}: {e}");
        Stdio::inherit()
    })
}

fn open_append(path: &str) -> Stdio {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map(Stdio::from)
        .unwrap_or_else(|e| {
            eprintln!("redirect >>: {path}: {e}");
            Stdio::inherit()
        })
}
