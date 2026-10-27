# gRPC Deep Dive with Rust

This project teaches gRPC with **tonic**, a popular Rust gRPC library, through four RPC patterns, error handling, and interceptors.

## Table of contents

- [0. What is gRPC?](#0-what-is-grpc-3-minutes)
- [1. Project structure](#1-project-structure)
- [2. Run the example](#2-run-the-example)
- [3. How it works](#3-how-it-works)
- [Lesson 1 — Unary RPC](#lesson-1--unary-rpc)
- [Lesson 2 — Error handling with `Status`](#lesson-2--error-handling-with-status)
- [Lesson 3 — Server streaming](#lesson-3--server-streaming)
- [Lesson 4 — Client streaming](#lesson-4--client-streaming)
- [Lesson 5 — Bidirectional streaming](#lesson-5--bidirectional-streaming)
- [Lesson 6 — Metadata, interceptors, and deadlines](#lesson-6--metadata-interceptors-and-deadlines)
- [Common problems](#common-problems)
- [Exercises](#exercises)

---

## 0. What is gRPC?

- **gRPC** is a way for two programs to call each other over a network, much like calling a local function.
- It runs over **HTTP/2**, and data is encoded with **Protocol Buffers (protobuf)**, which makes it compact, fast, and strongly typed.
- You define the API **once** in a `.proto` file, then generate server and client code (Rust, Go, Python, ...).

### REST/JSON vs gRPC

| Comparison   | REST/JSON            | gRPC                           |
|--------------|----------------------|--------------------------------|
| Format       | JSON (text)          | Protobuf (binary)              |
| API contract | OpenAPI (optional)   | `.proto` (required and strict) |
| Streaming    | Difficult            | Built in                       |
| Protocol     | HTTP/1.1 or HTTP/2   | HTTP/2                         |

### Four RPC patterns

| Pattern          | `.proto` syntax                     | Project example                         |
|------------------|-------------------------------------|-----------------------------------------|
| Unary            | `rpc A(Req) returns (Res)`          | `Greeter.SayHello`, `Calculator.Divide` |
| Server streaming | `returns (stream Res)`              | `Calculator.Fibonacci`                  |
| Client streaming | `(stream Req) returns (Res)`        | `Calculator.Sum`                        |
| Bidirectional    | `(stream Req) returns (stream Res)` | `Chat.Talk`                             |

---

## 1. Project structure

```text
grpc_deepdive/
├── Cargo.toml                 # dependencies
├── build.rs                   # compiles .proto into Rust code before compilation
├── proto/
│   └── deepdive.proto         # API contract (start here)
└── src/
    ├── lib.rs                 # includes generated code (the `pb` module)
    └── bin/
        ├── server.rs          # implements the services
        └── client.rs          # calls the services
```

---

## 2. Run the example

Install `protoc` first:

- Ubuntu: `sudo apt install protobuf-compiler`
- macOS: `brew install protobuf`

Then start the server and the client in two terminals:

```bash
cargo run --bin server      # terminal 1
cargo run --bin client      # terminal 2
```

> **Note:** With a newer Rust version (>= 1.85) you can delete `Cargo.lock` to resolve the latest library versions.
> The included `Cargo.lock` file pins dependency versions that work with Rust 1.75.

---

## 3. How it works

```text
deepdive.proto
      │
      │  build.rs + tonic-build
      ▼
Rust code in OUT_DIR
      │
      │  src/lib.rs: tonic::include_proto!("grpc_deepdive")
      ▼
module `pb`
      │
      ├───────────────────────────┐
      ▼                           ▼
server.rs                     client.rs
impl trait `Greeter`, ...     uses `GreeterClient`, ...
```

For each `service Foo` in the `.proto` file, tonic generates:

- `foo_server::Foo`: a **trait** that you implement.
- `foo_server::FooServer`: a wrapper used to attach the service to the server.
- `foo_client::FooClient`: a struct used to call methods.

For each `message`, tonic generates a Rust **struct** with the same name.

---

## Lesson 1 — Unary RPC

### Proto

```proto
service Greeter { rpc SayHello (HelloRequest) returns (HelloReply); }
message HelloRequest { string name = 1; }   // 1 is the field number, not the value!
```

### Server

Implement the trait:

```rust
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>)
        -> Result<Response<HelloReply>, Status>
    {
        let name = request.into_inner().name;          // extracts the message from the Request
        Ok(Response::new(HelloReply { message: format!("Hello, {name}!") }))
    }
}
```

**Key points:**

- `Request<T>` is message `T` plus metadata (headers). Call `into_inner()` to get `T`.
- The method returns `Result<Response<T>, Status>`: errors are always represented by `Status`.
- `#[tonic::async_trait]` allows `async fn` in a trait.

### Client

```rust
let mut client = GreeterClient::new(channel);
let reply = client.say_hello(HelloRequest { name: "Rustacean".into() }).await?;
println!("{}", reply.into_inner().message);
```

`client` needs to be mutable because the method takes `&mut self`. Cloning a `Channel` is inexpensive, so multiple clients can share one connection.

---

## Lesson 2 — Error handling with `Status`

gRPC has standard status codes (these are **not** HTTP status codes):

| Function                    | When to use                     |
|-----------------------------|---------------------------------|
| `Status::invalid_argument`  | The client sent invalid data    |
| `Status::not_found`         | The resource was not found      |
| `Status::unauthenticated`   | The client is unauthenticated   |
| `Status::permission_denied` | The client lacks permission     |
| `Status::out_of_range`      | The value is out of range       |
| `Status::internal`          | A server-side error             |

### Server

```rust
if b == 0.0 { return Err(Status::invalid_argument("cannot divide by zero")); }
```

### Client

The client can distinguish errors using `status.code()`:

```rust
match calc.divide(req).await {
    Ok(r) => ...,
    Err(status) if status.code() == Code::InvalidArgument => ...,
    Err(status) => return Err(status.into()),
}
```

---

## Lesson 3 — Server streaming

The server sends **multiple** messages for one request. There are three steps:

1. Declare `type FibonacciStream = ReceiverStream<Result<FibonacciReply, Status>>;`
2. Create an `mpsc` channel and `spawn` a task that sends data to `tx`.
3. Return `ReceiverStream::new(rx)`. The method returns immediately while data arrives over time.

### Server

```rust
let (tx, rx) = mpsc::channel(4);
tokio::spawn(async move {
    for index in 1..=count {
        if tx.send(Ok(FibonacciReply { index, value: a })).await.is_err() { break; } // client disconnected
        ...
    }
});
Ok(Response::new(ReceiverStream::new(rx)))
```

### Client

```rust
let mut stream = calc.fibonacci(FibonacciRequest { count: 10 }).await?.into_inner();
while let Some(item) = stream.message().await? { ... }
```

`message()` returns:

- `Ok(Some(x))` when data arrives,
- `Ok(None)` when the server finishes,
- `Err` on failure.

---

## Lesson 4 — Client streaming

The client sends **multiple** messages, and the server returns **one** result when the client finishes.

### Server

The server receives `Streaming<T>`:

```rust
let mut stream = request.into_inner();
while let Some(msg) = stream.message().await? { total += msg.value; }
```

### Client

The client converts an iterator into a stream:

```rust
let numbers = tokio_stream::iter(vec![5, 10, 15, 20]).map(|value| NumberRequest { value });
let reply = calc.sum(numbers).await?.into_inner();
```

---

## Lesson 5 — Bidirectional streaming

The two directions are **independent** and can be read and written concurrently. Chat is a typical example:

- **Server:** call both `inbound.message()` and `tx.send()` in one task.
- **Client:** one task sends messages to `tx`; the main task reads `inbound`.
- **Shutdown:** when `tx` is dropped, the outgoing stream ends, the server receives `Ok(None)`, and the loop stops.

---

## Lesson 6 — Metadata, interceptors, and deadlines

An **interceptor** works like middleware and runs before the handler.

### Server

Reject requests without a token:

```rust
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization").and_then(|v| v.to_str().ok()) {
        Some("Bearer secret-token") => Ok(req),
        _ => Err(Status::unauthenticated("token is missing or invalid")),
    }
}

GreeterServer::with_interceptor(MyGreeter, check_auth)
```

### Client

Automatically attach a token to every request:

```rust
GreeterClient::with_interceptor(channel, |mut req: Request<()>| {
    req.metadata_mut().insert("authorization", "Bearer secret-token".parse().unwrap());
    Ok::<_, Status>(req)
})
```

### Deadline

```rust
request.set_timeout(Duration::from_secs(2))
```

If the deadline passes, the client receives `Code::DeadlineExceeded`.

---

## Common problems

| Problem                                           | Cause / solution                                                                                         |
|---------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `Could not find protoc`                           | Install `protobuf-compiler` (or use the crate `protoc-bin-vendored`)                                     |
| `connection refused`                              | The server is not running                                                                                |
| `Unauthenticated`                                 | The Greeter client has not attached a token                                                              |
| Changes to `.proto` do not appear in generated code | Run `cargo build`. `build.rs` reruns when the `.proto` file changes if it has `cargo:rerun-if-changed` |

---

## Exercises

1. Add `rpc Multiply` to `Calculator` (edit the `.proto` file, implement it on the server, call it from the client).
2. Add field `string language = 2;` to `HelloRequest`, and reply "Hello" or "Xin chào" based on the language.
3. Make `Fibonacci` stop when the client cancels (observe the log `client disconnected`).
4. Turn `Chat` into a multi-user chat room using `tokio::sync::broadcast`.
5. Add TLS (`ServerTlsConfig`) and a health check (`tonic-health`).
6. Try calling the server using `grpcurl` (requires enabling `tonic-reflection`).
