use std::{
    collections::BTreeMap,
    net::SocketAddr,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use task_grpc::pb::{
    CreateTaskRequest, ImportSummary, ListTasksRequest, Task, WatchFilter,
    task_service_server::{TaskService, TaskServiceServer},
};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming, transport::Server};

struct TaskSvc {
    tasks: Mutex<BTreeMap<u64, Task>>,
    next_id: AtomicU64,
    events: broadcast::Sender<Task>,
}

impl TaskSvc {
    fn create(&self, req: CreateTaskRequest) -> Result<Task, Status> {
        let title = req.title.trim();
        if title.is_empty() {
            return Err(Status::invalid_argument("title must not be empty"));
        }
        if title.len() > 200 {
            return Err(Status::invalid_argument("title too long (max 200 bytes)"));
        }

        let task = Task {
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            title: title.to_owned(),
            tags: req.tags,
            done: false,
        };

        self.tasks.lock().unwrap().insert(task.id, task.clone());
        let _ = self.events.send(task.clone());
        Ok(task)
    }
}

#[tonic::async_trait]
impl TaskService for TaskSvc {
    // 1. Unary
    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<Task>, Status> {
        self.create(request.into_inner()).map(Response::new)
    }

    // 2. Server streaming
    type ListTasksStream = ReceiverStream<Result<Task, Status>>;

    async fn list_tasks(
        &self,
        request: Request<ListTasksRequest>,
    ) -> Result<Response<Self::ListTasksStream>, Status> {
        let tag = request.into_inner().tag;

        let snapshot: Vec<Task> = self
            .tasks
            .lock()
            .unwrap()
            .values()
            .filter(|t| tag.is_empty() || t.tags.contains(&tag))
            .cloned()
            .collect();

        let (tx, rx) = mpsc::channel(8);
        tokio::spawn(async move {
            for task in snapshot {
                if tx.send(Ok(task)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    // 3. Client streaming
    async fn import_tasks(
        &self,
        request: Request<Streaming<CreateTaskRequest>>,
    ) -> Result<Response<ImportSummary>, Status> {
        let mut stream = request.into_inner();
        let (mut created, mut rejected) = (0, 0);

        while let Some(req) = stream.message().await? {
            match self.create(req) {
                Ok(_) => created += 1,
                Err(_) => rejected += 1,
            }
        }

        Ok(Response::new(ImportSummary { created, rejected }))
    }

    // 4. Bidirectional streaming
    type WatchStream = ReceiverStream<Result<Task, Status>>;

    async fn watch(
        &self,
        request: Request<Streaming<WatchFilter>>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let mut inbound = request.into_inner();
        let mut events = self.events.subscribe();
        let (out_tx, out_rx) = mpsc::channel(16);

        tokio::spawn(async move {
            let mut tag = String::new();
            loop {
                tokio::select! {
                    // client sends new filter or closes stream
                    msg = inbound.message() => match msg {
                        Ok(Some(filter)) => tag = filter.tag,
                        Ok(None) | Err(_) => break,
                    },
                    // new task is created
                    ev = events.recv() => match ev {
                        Ok(task) => {
                            if (tag.is_empty() || task.tags.contains(&tag))
                                && out_tx.send(Ok(task)).await.is_err() {
                                break;
                            }
                        }
                        // Consumer is too slow
                        Err(broadcast::error::RecvError::Lagged(_)) => {}
                        Err(broadcast::error::RecvError::Closed) => break,
                    },
                    // Client leaves while idling: remove task and cleanup
                    _ = out_tx.closed() => break,
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(out_rx)))
    }

    // Interceptor: run before any handler, only read metadata (cannot read message)
}

fn auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization").map(|v| v.to_str()) {
        Some(Ok("Bearer secret-token")) => Ok(req),
        _ => Err(Status::unauthenticated("missing or invalid token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let (events, _) = broadcast::channel(64);

    let svc = TaskSvc {
        tasks: Mutex::default(),
        next_id: AtomicU64::new(1),
        events,
    };

    println!("listening on {addr}");
    Server::builder()
        .http2_keepalive_interval(Some(Duration::from_secs(30)))
        .add_service(TaskServiceServer::with_interceptor(svc, auth))
        .serve_with_shutdown(addr, async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
