fn main() {
    // Snippet A
    let fut = async { 42 };
    let pinned = Box::pin(fut);
    let moved = pinned; // Move the Box
    let result = moved.await;

    // Snippet B
    let fut = async { 42 };
    tokio::pin!(fut);
    let moved = fut; // Move the pinned future
    let result = moved.await;

    // Snippet C
    use std::pin::Pin;
    let mut fut = async { 42 };
    let pinned = Pin::new(&mut fut);
}
