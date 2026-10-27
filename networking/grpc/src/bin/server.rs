//! gRPC server - run with: cargo run --bin server

use std::time::Duration;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming, transport::Server};

use grpc::pb::{
    ChatMessage, DivideReply, DivideRequest, FibonacciReply, FibonacciRequest, HelloReply,
    HelloRequest, NumberRequest, SumReply,
    calculator_server::{Calculator, CalculatorServer},
    chat_server::{Chat, ChatServer},
    greeter_server::{Greeter, GreeterServer},
};

// ============================================================
// LESSON 1: UNARY RPC (Greeter)
// Each service in .proto becomes a Rust trait, which we implement here.
// ============================================================
#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait] // Allows `async fn` in a trait
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        // Metadata is like an HTTP header and can be read from the request.
        println!("[Greeter] metadata: {:?}", request.metadata());

        // into_inner() extracts the message from the Request wrapper.
        let name = request.into_inner().name;

        // Return errors with Status, which uses standard gRPC status codes.
        if name.trim().is_empty() {
            return Err(Status::invalid_argument("name must not be empty"));
        }

        Ok(Response::new(HelloReply {
            message: format!("Hello, {name}!"),
        }))
    }
}

// ============================================================
// LESSONS 2 AND 3: Calculator (unary, server streaming, and client streaming)
// ============================================================
#[derive(Default)]
struct MyCalculator;

#[tonic::async_trait]
impl Calculator for MyCalculator {
    // ---- Unary RPC and error handling ----
    async fn divide(
        &self,
        request: Request<DivideRequest>,
    ) -> Result<Response<DivideReply>, Status> {
        let DivideRequest { a, b } = request.into_inner();
        if b == 0.0 {
            return Err(Status::invalid_argument("cannot divide by zero"));
        }
        Ok(Response::new(DivideReply { result: a / b }))
    }

    // ---- Server streaming ----
    // Declare the Stream type returned by this method.
    // ReceiverStream wraps a channel: we send through `tx`, and the client receives from `rx`.
    type FibonacciStream = ReceiverStream<Result<FibonacciReply, Status>>;

    async fn fibonacci(
        &self,
        request: Request<FibonacciRequest>,
    ) -> Result<Response<Self::FibonacciStream>, Status> {
        let count = request.into_inner().count;
        if count == 0 || count > 90 {
            // 90 is the limit that prevents u64 overflow.
            return Err(Status::out_of_range("count must be in the range 1..=90"));
        }

        let (tx, rx) = mpsc::channel(4);

        // Generate values in a separate task so this method can return immediately.
        tokio::spawn(async move {
            let (mut a, mut b) = (0u64, 1u64);
            for index in 1..=count {
                let reply = FibonacciReply { index, value: a };
                // A send error means the client disconnected, so stop early.
                if tx.send(Ok(reply)).await.is_err() {
                    println!("[Fibonacci] client disconnected");
                    break;
                }
                (a, b) = (b, a + b);
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    // ---- Client streaming ----
    // Request contains `Streaming<T>`: read each message sent by the client.
    async fn sum(
        &self,
        request: Request<Streaming<NumberRequest>>,
    ) -> Result<Response<SumReply>, Status> {
        let mut stream = request.into_inner();
        let (mut count, mut total) = (0u32, 0i64);

        // message() -> Ok(Some(msg)) | Ok(None) when the client is done | Err
        while let Some(msg) = stream.message().await? {
            println!("[Sum] received {}", msg.value);
            count += 1;
            total += msg.value;
        }

        Ok(Response::new(SumReply { count, total }))
    }
}

// ============================================================
// LESSON 4: Bidirectional streaming (Chat)
// ============================================================
#[derive(Default)]
struct MyChat;

#[tonic::async_trait]
impl Chat for MyChat {
    type TalkStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn talk(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::TalkStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = mpsc::channel(8);

        tokio::spawn(async move {
            loop {
                match inbound.message().await {
                    Ok(Some(msg)) => {
                        println!("[Chat] {}: {}", msg.user, msg.text);
                        // The bot replies to every message.
                        let reply = ChatMessage {
                            user: "Bot".into(),
                            text: format!("Bot received: \"{}\"", msg.text),
                        };
                        if tx.send(Ok(reply)).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break, // The client closed the stream.
                    Err(e) => {
                        eprintln!("[Chat] error: {e}");
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// ============================================================
// LESSON 5: Interceptor middleware for checking a token
// ============================================================
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = req
        .metadata()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    match token {
        Some("Bearer secret-token") => Ok(req),
        _ => Err(Status::unauthenticated("token is missing or invalid")),
    }
}

// ============================================================
// Main function: attach all services to one server
// ============================================================
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    println!("Server is running at {addr}");

    Server::builder()
        // Greeter uses an interceptor, so a token is required.
        .add_service(GreeterServer::with_interceptor(MyGreeter, check_auth))
        // The other two services do not require a token.
        .add_service(CalculatorServer::new(MyCalculator))
        .add_service(ChatServer::new(MyChat))
        .serve(addr)
        .await?;

    Ok(())
}
