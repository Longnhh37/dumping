use std::time::Duration;

use task_grpc::pb::{
    CreateTaskRequest, ListTasksRequest, WatchFilter, task_service_client::TaskServiceClient,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    Code, Request, Status,
    metadata::{Ascii, MetadataValue},
    transport::Endpoint,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://[::1]:50051")
        .connect_timeout(Duration::from_secs(3))
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .connect()
        .await?;

    let mut anon = TaskServiceClient::new(channel.clone());
    if let Err(e) = anon.list_tasks(ListTasksRequest::default()).await {
        println!("[auth] {:?}: {}", e.code(), e.message());
    }

    let token: MetadataValue<Ascii> = "Bearer secret-token".parse()?;
    let mut client = TaskServiceClient::with_interceptor(
        channel,
        move |mut req: Request<()>| -> Result<Request<()>, Status> {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    // ---------- 1. Unary + deadline ----------
    let mut req = Request::new(CreateTaskRequest {
        title: "Learn gRPC".into(),
        tags: vec!["study".into()],
    });
    req.set_timeout(Duration::from_secs(2));
    let task = client.create_task(req).await?.into_inner();
    println!("[unary] created {task:?}");

    let bad = CreateTaskRequest { title: "   ".into(), tags: vec![] };
    match client.create_task(bad).await {
        Err(s) if s.code() == Code::InvalidArgument => {
            println!("[unary] rejected: {}", s.message());
        }
        other => println!("[unary] unexpected: {other:?}"),
    }

    // ---------- 2. Client streaming ----------
    let titles = ["Buy milk", "Write report", "", "Read RFC 9113"];
    let reqs = tokio_stream::iter(titles.map(|t| CreateTaskRequest {
        title: t.into(),
        tags: vec!["import".into()],
    }));
    let summary = client.import_tasks(reqs).await?.into_inner();
    println!("[client-stream] {summary:?}");

    // ---------- 3. Server streaming ----------
    let mut stream = client
        .list_tasks(ListTasksRequest { tag: String::new() })
        .await?
        .into_inner();
    while let Some(task) = stream.message().await? {
        println!("[server-stream] #{} {}", task.id, task.title);
    }

    // ---------- 4. Bidirectional streaming ----------
    let (filter_tx, filter_rx) = mpsc::channel(4);
    filter_tx.send(WatchFilter { tag: "urgent".into() }).await?;
    let mut events = client
        .watch(ReceiverStream::new(filter_rx))
        .await?
        .into_inner();

    let mut producer = client.clone();
    tokio::spawn(async move {
        for (i, tag) in ["urgent", "later", "urgent"].into_iter().enumerate() {
            tokio::time::sleep(Duration::from_millis(300)).await;
            let req = CreateTaskRequest {
                title: format!("live task #{i}"),
                tags: vec![tag.into()],
            };
            let _ = producer.create_task(req).await;
        }
    });

    if let Some(t) = events.message().await? {
        println!("[bidi] got {} {:?}", t.title, t.tags);
    }
    filter_tx.send(WatchFilter { tag: "later".into() }).await?;
    if let Some(t) = events.message().await? {
        println!("[bidi] got {} {:?}", t.title, t.tags);
    }

    drop(filter_tx);
    Ok(())
}
