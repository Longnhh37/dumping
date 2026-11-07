# Capstone Project: A Simple Task-Scheduler

A small, type-safe, concurrent task scheduler written in Rust (edition 2024).
Tasks move through a lifecycle that is enforced **at compile time**, a fixed pool
of worker threads executes them in parallel, and every outcome is collected
without a single unhandled panic.

This project is based on the capstone chapter of
[Rust Patterns & Engineering How-Tos](https://microsoft.github.io/RustTraining/rust-patterns-book/ch19-capstone-project.html),
with a few design changes described in [Design decisions](#design-decisions).

## Features

- **Typestate lifecycle**: `Pending → Running → Completed | Failed`. Invalid
  transitions (for example re-running a finished task) do not compile.
- **Outcome travels with the state**: `Completed<R>` carries the value and
  `Failed` carries the error, so a finished task cannot exist without its outcome.
- **Worker pool**: N threads pull work from one channel and run tasks in parallel.
- **Panic-safe**: a panicking task is caught and reported as a failed task; the
  worker keeps running.
- **Clean shutdown**: `shutdown()` waits for all queued tasks and returns every
  result, sorted by task ID. Dropping a scheduler also joins its workers.
- **No `unsafe`**: the crate uses `#![forbid(unsafe_code)]`.

## Requirements

- Rust 1.90 or newer (edition 2024)

## Quick start

```bash
cargo run --release     # demo: 20 tasks on 4 workers
cargo test              # unit + integration + property tests + doctests
cargo doc --no-deps --open
```

### Usage

```rust
use task_scheduler::{Scheduler, WorkItem};

fn main() {
    // Spawn 4 worker threads.
    let scheduler = Scheduler::<u64>::new(4);

    for i in 0..10u64 {
        let item = WorkItem::new(format!("square-{i}"), move || {
            if i == 7 { Err("unlucky number".to_string()) } else { Ok(i * i) }
        });
        scheduler.submit(item).expect("scheduler is open");
    }

    // Waits for every task, then returns all results sorted by task ID.
    for result in scheduler.shutdown() {
        match result.outcome() {
            Ok(value) => println!("{} -> {value}", result.name()),
            Err(error) => println!("{} failed: {error}", result.name()),
        }
    }
}
```

## Project structure

```
task-scheduler/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs          # public API re-exports
│   ├── main.rs         # demo binary
│   ├── id.rs           # TaskId newtype + atomic ID generator
│   ├── task.rs         # typestate: Task<S>, Pending/Running/Completed<R>/Failed
│   ├── work.rs         # WorkItem<R> = Task<Pending> + boxed closure
│   ├── scheduler.rs    # Scheduler<R>, TaskResult<R>, worker loop
│   └── error.rs        # SchedulerError (thiserror)
└── tests/
    ├── integration.rs  # happy path, failures, panics, parallelism, close
    └── properties.rs   # proptest: N tasks in -> N results out
```

## Architecture

### Module dependencies

```
             ┌───────────┐
             │   id.rs   │  TaskId
             └─────┬─────┘
           ┌───────┴────────┐
           ▼                ▼
    ┌────────────┐    ┌────────────┐
    │  task.rs   │    │  error.rs  │  SchedulerError
    │  Task<S>   │    └──────┬─────┘
    └─────┬──────┘           │
          ▼                  │
    ┌────────────┐           │
    │  work.rs   │           │
    │ WorkItem<R>│           │
    └─────┬──────┘           │
          └──────────┬───────┘
                     ▼
              ┌──────────────┐
              │ scheduler.rs │
              │ Scheduler<R> │
              └──────────────┘
```

Each layer only knows about the layers below it.

### Task lifecycle

```
Task<Pending> ──start()──▶ Task<Running> ──complete(value)──▶ Task<Completed<R>>
                                │
                                └────────fail(error)─────────▶ Task<Failed>
```

| State          | Carries         | Memory cost       |
|----------------|-----------------|-------------------|
| `Pending`      | nothing         | zero-sized        |
| `Running`      | nothing         | zero-sized        |
| `Completed<R>` | the value `R`   | size of `R`       |
| `Failed`       | error `String`  | size of `String`  |

Transitions consume `self`, so the old state cannot be reused after a transition.

### Data flow

```
  Caller thread            Work channel           Worker threads
  ──────────────            ────────────           ──────────────
  WorkItem::new(...)
  submit(item) ───────────▶ recv() ──────────────▶ run(item)
                                                    │
                                                    ├─ start task
                                                    ├─ run job under catch_unwind
                                                    ├─ complete(value) or fail(error)
                                                    └─ build TaskResult
       ▲                                            │
       │                 Result channel             │
       └──── collect ◀── send(TaskResult) ◀─────────┘

  shutdown(): close work channel → join workers → collect and sort results
```

There are two channels: a **work channel** (caller → workers, carries `WorkItem`)
and a **result channel** (workers → caller, carries `TaskResult`).

### `Task` vs `WorkItem` vs `TaskResult`

| Type            | Role                                                        | Lives in                          |
|-----------------|-------------------------------------------------------------|-----------------------------------|
| `Task<S>`       | Identity (`id`, `name`) plus lifecycle state in the type    | mostly inside `run()` on a worker |
| `WorkItem<R>`   | A `Task<Pending>` plus the closure to execute               | caller → channel → worker         |
| `TaskResult<R>` | Plain runtime data: `id`, `name`, `outcome: Result<R, String>` | worker → channel → caller      |

`TaskResult` has no typestate on purpose: results from different tasks are
collected into one `Vec`, which needs a single element type.

## Design decisions

- **Outcome inside the state.** `complete(value)` and `fail(error)` require the
  outcome, and `TaskResult` can only be built from a finished task via
  `From<Task<Completed<R>>>` / `From<Task<Failed>>` (its fields are private).
- **Sealed state types.** `TaskState` is sealed and the state structs have a
  private field, so code outside the crate cannot invent new states or build a
  `Task` in an arbitrary state.
- **`Task` and `WorkItem` are separate.** The closure is consumed when it runs
  (`FnOnce`), while the lifecycle must be tracked across the whole execution.
  `into_parts()` (crate-private) lets a worker take the closure and keep the task.
- **Shared receiver behind `Arc<Mutex<_>>`.** `mpsc::Receiver` is not `Clone`.
  Workers hold the lock only while waiting for the next item, never while
  running a task, otherwise tasks would run one at a time.
- **Panics become failures.** Each job runs under `catch_unwind`, so one bad task
  cannot take a worker down.
- **Regular threads, not scoped threads.** The scheduler outlives individual
  `submit` calls, so workers are spawned with `thread::Builder` (named
  `worker-N`). Scoped threads are used in tests instead.
- **Deterministic output.** `shutdown()` sorts results by `TaskId`, because
  completion order depends on thread scheduling.
- **`SchedulerError` differs from the book.** `ChannelError(SendError<()>)` was
  replaced by `ChannelClosed` because `submit` sends `WorkItem<R>`, and
  `WorkerPanic` was dropped because task panics are already captured.

## Testing

| Test                                      | What it checks                                           |
|-------------------------------------------|----------------------------------------------------------|
| `happy_path`                              | 10 tasks → 10 `Ok` results, ordered by ID                |
| `handles_failures`                        | A failing task yields `SchedulerError::TaskFailed`       |
| `empty_scheduler_shuts_down_cleanly`      | No panic, no results                                     |
| `submit_after_close_is_rejected`          | `submit` returns `ShutDown` after `close()`              |
| `panicking_task_is_captured_as_failure`   | Panic message ends up in the failed result               |
| `workers_run_in_parallel`                 | At least two tasks run at the same time                  |
| `returns_exactly_n_results` (proptest)    | For any N, workers, and failure pattern: N results, same IDs, correct failure count |
| `compile_fail` doctest on `Task`          | Calling `start()` on a completed task does not compile   |

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Known limitations

- The typestate protects the *shape* of the lifecycle, not the choice made at the
  end of it. For example, if `R = String`, writing `Ok(v) => running.fail(v)` by
  mistake still compiles. The short `match` in `run()` plus the tests cover this.
- Tasks are processed in FIFO order with no priorities.
- A task cannot be cancelled once submitted.
- Failed tasks are not retried, and errors are plain `String`s.

## Extension ideas

1. **Priority queue**: a `Priority` newtype (1–10, validated with `TryFrom<u8>`),
   replacing the channel with a `BinaryHeap` guarded by a `Condvar`.
2. **Retry policy**: retry a failed task up to N times (requires `Fn`/`FnMut` or a
   job factory instead of `FnOnce`).
3. **Cancellation**: `cancel(TaskId)` to remove pending tasks (needs a queue the
   scheduler owns).
4. **Async version**: port to `tokio::spawn` with `tokio::sync::mpsc`.
5. **Metrics**: per-worker task counts, average duration, failure rate.
6. **Typed errors**: replace `String` errors with a generic error type.

## Concepts practiced

Generics and trait bounds · typestate pattern · sealed traits · newtypes ·
`Box<dyn FnOnce>` · channels (`mpsc`) · `Arc<Mutex<_>>` · atomics ·
`catch_unwind` · `thiserror` · `Drop` · scoped threads · property-based testing ·
doctests (`compile_fail`).
