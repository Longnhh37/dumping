//! gRPC client - run after starting the server: cargo run --bin client

use std::time::Duration;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Channel, Code, Request, Status};

use grpc::pb::{
    calculator_client::CalculatorClient, chat_client::ChatClient, greeter_client::GreeterClient,
    ChatMessage, DivideRequest, FibonacciRequest, HelloRequest, NumberRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A Channel is one HTTP/2 connection; it is cheap to clone and share across clients.
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    demo_unary(channel.clone()).await?;
    demo_errors(channel.clone()).await?;
    demo_server_streaming(channel.clone()).await?;
    demo_client_streaming(channel.clone()).await?;
    demo_bidirectional(channel.clone()).await?;
    Ok(())
}

// ---------- Lessons 1 and 5: unary RPC and sending a token with an interceptor ----------
async fn demo_unary(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 1. UNARY (with token) ===");

    // Client interceptor: add the "authorization" header to every request.
    let mut greeter = GreeterClient::with_interceptor(channel, |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", "Bearer secret-token".parse().unwrap());
        Ok::<_, Status>(req)
    });

    // Set a deadline: if no response arrives within 2 seconds, return DeadlineExceeded.
    let mut request = Request::new(HelloRequest { name: "Rustacean".into() });
    request.set_timeout(Duration::from_secs(2));

    let response = greeter.say_hello(request).await?;
    println!("Server replied: {}", response.into_inner().message);
    Ok(())
}

// ---------- Lesson 2: Error handling ----------
async fn demo_errors(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 2. ERROR HANDLING ===");

    // a) Omit the token -> Unauthenticated.
    let mut greeter = GreeterClient::new(channel.clone()); // No interceptor.
    match greeter.say_hello(HelloRequest { name: "Anon".into() }).await {
        Ok(_) => println!("(unexpected)"),
        Err(status) => println!("a) code = {:?}, message = {}", status.code(), status.message()),
    }

    // b) Divide by zero -> InvalidArgument.
    let mut calc = CalculatorClient::new(channel);
    match calc.divide(DivideRequest { a: 10.0, b: 0.0 }).await {
        Ok(r) => println!("b) result: {}", r.into_inner().result),
        Err(status) if status.code() == Code::InvalidArgument => {
            println!("b) input error: {}", status.message())
        }
        Err(status) => return Err(status.into()),
    }

    // c) Successful case.
    let r = calc.divide(DivideRequest { a: 10.0, b: 4.0 }).await?;
    println!("c) 10 / 4 = {}", r.into_inner().result);
    Ok(())
}

// ---------- Lesson 3: Server streaming ----------
async fn demo_server_streaming(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 3. SERVER STREAMING (Fibonacci) ===");
    let mut calc = CalculatorClient::new(channel);

    let mut stream = calc
        .fibonacci(FibonacciRequest { count: 10 })
        .await?
        .into_inner();

    // Receive each item as soon as the server sends it; there is no need to wait for the full stream.
    while let Some(item) = stream.message().await? {
        println!("fib[{}] = {}", item.index, item.value);
    }
    Ok(())
}

// ---------- Lesson 4: Client streaming ----------
async fn demo_client_streaming(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 4. CLIENT STREAMING (Sum) ===");
    let mut calc = CalculatorClient::new(channel);

    // Convert a regular iterator into a stream to send to the server.
    let numbers = tokio_stream::iter(vec![5, 10, 15, 20]).map(|value| NumberRequest { value });

    let reply = calc.sum(numbers).await?.into_inner();
    println!("Sent {} numbers, total = {}", reply.count, reply.total);
    Ok(())
}

// ---------- Lesson 5: Bidirectional streaming ----------
async fn demo_bidirectional(channel: Channel) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 5. BIDIRECTIONAL STREAMING (Chat) ===");
    let mut chat = ChatClient::new(channel);

    // Use a channel to send and receive at the same time.
    let (tx, rx) = mpsc::channel(8);

    tokio::spawn(async move {
        for text in ["Hello, bot", "Learning gRPC today", "Goodbye"] {
            let msg = ChatMessage { user: "Alice".into(), text: text.into() };
            println!("Alice sent: {}", msg.text);
            if tx.send(msg).await.is_err() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        // tx is dropped here, which ends the outgoing stream.
    });

    let mut inbound = chat.talk(ReceiverStream::new(rx)).await?.into_inner();
    while let Some(msg) = inbound.message().await? {
        println!("{} replied: {}", msg.user, msg.text);
    }
    println!("Chat ended.");
    Ok(())
}
