use std::os::fd::{AsFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::process::{Command as StdCommand, Stdio};

use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{ForkResult, Pid, dup2_stderr, dup2_stdin, dup2_stdout, fork, pipe};

use crate::builtin;
use crate::exec::redirect::{make_stdio_set, make_writer_set, open_redirect_fds};
use crate::exec::resolve::{CommandKind, resolve};
use crate::expander::ExpandedCommand;
use crate::runtime::{ExecStatus, ShellState};

pub fn run(cmds: Vec<ExpandedCommand>, state: &mut ShellState) -> ExecStatus {
    let len = cmds.len();
    let mut prev_read: Option<OwnedFd> = None;
    let mut pids = Vec::<Pid>::new();
    let mut last_pid: Option<Pid> = None;

    for (i, cmd) in cmds.into_iter().enumerate() {
        let is_last = i == len - 1;

        match resolve(&cmd.name) {
            // ─────────────────────────────────────────────────────────────────
            // External command
            // ─────────────────────────────────────────────────────────────────
            CommandKind::External { name, path } => {
                // Start from the file-redirect defaults for all three streams.
                let mut stdio = make_stdio_set(&cmd.redirect);

                // Pipeline overrides take priority over file redirects:
                //   • stdin  → previous pipe segment's read-end
                //   • stdout → new pipe write-end (when not last command)
                if let Some(fd) = prev_read.take() {
                    stdio.stdin = Stdio::from(fd);
                }
                if !is_last {
                    stdio.stdout = Stdio::piped();
                }

                match StdCommand::new(&path)
                    .arg0(&name)
                    .args(&cmd.args)
                    .stdin(stdio.stdin)
                    .stdout(stdio.stdout)
                    .stderr(stdio.stderr)
                    .spawn()
                {
                    Ok(mut child) => {
                        if !is_last {
                            prev_read = child.stdout.take().map(OwnedFd::from);
                        }
                        let pid = Pid::from_raw(child.id() as i32);
                        last_pid = Some(pid);
                        pids.push(pid);
                    }
                    Err(e) => {
                        eprintln!("{}: {}", cmd.name, e);
                        return ExecStatus::Code(1);
                    }
                }
            }

            // ─────────────────────────────────────────────────────────────────
            // Builtin command  —  fork so it can participate in a pipeline
            // ─────────────────────────────────────────────────────────────────
            CommandKind::Builtin => {
                // Pipe for stdout→next-stage (only needed when not last).
                let (pipe_r, pipe_w) = if !is_last {
                    match pipe() {
                        Ok((r, w)) => (Some(r), Some(w)),
                        Err(e) => {
                            eprintln!("pipe failed: {}", e);
                            return ExecStatus::Code(1);
                        }
                    }
                } else {
                    (None, None)
                };

                // Open any file-backed redirects BEFORE fork so that errors
                // are visible in the parent and the fds exist in both processes.
                let rfds = open_redirect_fds(&cmd.redirect);

                match unsafe { fork() } {
                    Ok(ForkResult::Child) => {
                        // ── stdin ──────────────────────────────────────────
                        // Pipe from previous stage takes priority; fall back to
                        // a file redirect (<).
                        if let Some(fd) = prev_read.take() {
                            if dup2_stdin(fd.as_fd()).is_err() {
                                eprintln!("dup2 stdin (pipe) failed");
                                std::process::exit(1);
                            }
                        } else if let Some(fd) = rfds.stdin
                            && dup2_stdin(fd.as_fd()).is_err()
                        {
                            eprintln!("dup2 stdin (redirect) failed");
                            std::process::exit(1);
                        }

                        // ── stdout ─────────────────────────────────────────
                        // Next-stage pipe takes priority; fall back to a file
                        // redirect (> or >>).
                        if let Some(ref w) = pipe_w
                            && dup2_stdout(w.as_fd()).is_err()
                        {
                            eprintln!("dup2 stdout (pipe) failed");
                            std::process::exit(1);
                        } else if let Some(fd) = rfds.stdout
                            && dup2_stdout(fd.as_fd()).is_err()
                        {
                            eprintln!("dup2 stdout (redirect) failed");
                            std::process::exit(1);
                        }

                        // ── stderr ─────────────────────────────────────────
                        // No pipe for stderr, only apply file redirects versions
                        if let Some(fd) = rfds.stderr
                            && dup2_stderr(fd.as_fd()).is_err()
                        {
                            eprintln!("dup2 stderr (redirect) failed");
                            std::process::exit(1);
                        }

                        drop(pipe_r);
                        drop(pipe_w);
                        drop(prev_read);

                        let builtin = builtin::get(&cmd.name).unwrap();

                        let mut writers = match make_writer_set(&cmd.redirect) {
                            Ok(w) => w,
                            Err(e) => {
                                eprintln!("{}: {}", cmd.name, e);
                                return ExecStatus::Code(1);
                            }
                        };

                        let status = builtin.run(
                            &cmd.args,
                            state, 
                            &mut writers.stdout,
                            &mut writers.stderr
                        );

                        match status {
                            ExecStatus::Code(c) | ExecStatus::Exit(c) => std::process::exit(c),
                        }
                    }

                    Ok(ForkResult::Parent { child }) => {
                        drop(pipe_w);
                        prev_read = pipe_r;
                        last_pid = Some(child);
                        pids.push(child);
                    }

                    Err(e) => {
                        eprintln!("fork failed: {}", e);
                        return ExecStatus::Exit(1);
                    }
                }
            }

            // ─────────────────────────────────────────────────────────────────
            // Not found
            // ─────────────────────────────────────────────────────────────────
            CommandKind::NotFound => {
                eprintln!("{}: command not found", cmd.name);
                return ExecStatus::Code(127);
            }
        }
    }

    drop(prev_read);

    let mut last_status = ExecStatus::Code(0);

    for pid in pids {
        let status = wait_for(pid);
        if Some(pid) == last_pid {
            last_status = status;
        }
    }

    last_status
}

fn wait_for(pid: Pid) -> ExecStatus {
    loop {
        match waitpid(pid, None) {
            Ok(WaitStatus::Exited(_, code)) => return ExecStatus::Code(code),
            Ok(WaitStatus::Signaled(_, sig, _)) => return ExecStatus::Code(128 + sig as i32),
            Ok(_) => {}
            Err(nix::errno::Errno::EINTR) => continue,
            Err(e) => {
                eprintln!("waitpid failed: {}", e);
                return ExecStatus::Code(1);
            }
        }
    }
}
