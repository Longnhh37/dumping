fn parallel_map<T: Sync, R: Send>(data: &[T], f: fn(&T) -> R, num_threads: usize) -> Vec<R> {
    let chunk_size = data.len().div_ceil(num_threads).max(1);
    let mut res = Vec::with_capacity(data.len());

    std::thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().map(f).collect::<Vec<R>>()))
            .collect();

        for h in handles {
            res.extend(h.join().unwrap());
        }
    });

    res
}
