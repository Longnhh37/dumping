use std::sync::{Arc, Mutex, mpsc};
use std::thread;

struct Job {
    id: u64,
    data: String,
}

struct JobResult {
    job_id: u64,
    output: String,
    worker_id: usize,
}

fn worker_pool(jobs: Vec<Job>, num_workers: usize) -> Vec<JobResult> {
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (res_tx, res_rx) = mpsc::channel::<JobResult>();

    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut handles = Vec::new();
    for worker_id in 0..num_workers {
        let job_rx = job_rx.clone();
        let res_tx = res_tx.clone();
        handles.push(thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(job) => {
                        let output = format!("processed '{}' by worker {worker_id}", job.data);
                        res_tx
                            .send(JobResult {
                                job_id: job.id,
                                output,
                                worker_id,
                            })
                            .unwrap();
                    }
                    Err(_) => break,
                }
            }
        }));
    }
    drop(res_tx);

    let num_jobs = jobs.len();
    for job in jobs {
        job_tx.send(job).unwrap();
    }
    drop(job_tx);

    let res: Vec<_> = res_rx.into_iter().collect();
    assert_eq!(res.len(), num_jobs);

    for h in handles { h.join().unwrap() }
    res
}
