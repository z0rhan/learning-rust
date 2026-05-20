use std::pin::Pin;
use std::task::{Context, Poll};


pub trait Future {
    // what the future resolves to
    // similar to Item type associated with Iterator trait
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    // Poll is similar to Option with Pending and Ready(T) variants
    // where T is the value available after future has finised its work

    // So rust compiler compiles every .await to roughly,
    // match future.poll() {
    // Ready(val) => ..., // val is the return type of the future
    // Pending => {
    //   // continue
    // }
    // }
    // But it would need to repeatedly check if the return value is Ready or not
    // But it would not be a loop since then it would be a blocking operation
    // This is the job of async runtime

    // Pin and Unpin
    // So when a obj has internal reference and when the obj is moved, the
    // internal ref will still point to the old mem addr which then violates
    // Rust's memory model. Hence we have Pin wrapper which guarantees that
    // that the object will not move as to not violate the memory model by
    // internal ref, It is wrapper for poiter-like types (types that implement
    // Deref or DerefMut)
    // But, then most types are safe to move around and hence we have the marker
    // triat Unpin, which tells the compiler that the value in question does
    // not need to be pinned. The compiler implements this trait for us
    // But if the value in question cannot be moved, it will have the !Unpin trait
    // which cannot guarantee that the value can be moved safely and hence needs
    // to be pinned
}

// For iterator, we have the next() method which provides Option<Self::Item>
// For future, we poll() which provides an idea of readiness over time as
// Poll<Self::Item>
// Now, for stream i.e. a seq of items that become ready over time, we can define
// But as of now not part of the std lib
pub trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

pub trait SteamExt : Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;
}
