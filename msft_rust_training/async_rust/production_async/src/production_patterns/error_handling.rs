// Error context is lost across spawn boundaries:
async fn bad_error_handling() -> Result<()> {
    let handle = tokio::spawn(async {
        some_fallible_work().await // Returns Result<T, SomeError>
    });

    // handle.await returns Result<Result<T, SomeError>, JoinError>
    // The inner error has no context about what task failed
    let result = handle.await??;
    Ok(())
}

// Add context at the spawn boundary:
async fn good_error_handling() -> Result<()> {
    let handle = tokio::spawn(async {
        some_fallible_work().await.context("worker task failed") // Context before crossing boundary
    });

    let result = handle.await.context("worker task panicked")??; // Context for JoinError too
    Ok(())
}
