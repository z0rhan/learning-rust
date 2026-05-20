use async_traits::Future;

struct MockObj {
    str: String
}

impl Future for MockObj {
    type Output = String;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        
    }
}

fn main() {
    println!("Hello, world!");
}
