fn main() {
    /*
    Snippet A
    Compiles. Box::pin() puts the future on the heap.
    Moving the Box moves the pointer, not the future itself.
    The future stays pinned in its heap location.
    */
    let fut = async { 42 };
    let pinned = Box::pin(fut);
    let moved = pinned; // Move the Box
    let result = moved.await;

    /*
    Snippet B
    Compiles. tokio::pin! pins the future to the stack and rebinds fut as Pin<&mut ...>.
    let moved = fut moves the Pin wrapper (a pointer),
        not the underlying future — the future stays pinned on the stack.
    This is just like Box::pin: moving the Box doesn’t move the heap allocation.
    However, fut is consumed by the move, so you can’t use fut afterwards — only moved:
    */
    let fut = async { 42 };
    tokio::pin!(fut);
    let moved = fut; // Move the pinned future
    let result = moved.await;

    /*
    Snippet C
    Does not compile.
    Pin::new() requires T: Unpin. Async blocks generate !Unpin types.
    Fix: Use Box::pin() or unsafe Pin::new_unchecked():
    */
    use std::pin::Pin;
    let mut fut = async { 42 };
    let pinned = Pin::new(&mut fut);
}
