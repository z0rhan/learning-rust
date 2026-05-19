use std::time::Duration;
use trpl::{Either};

pub async fn timeout<F: Future>(
    f: F,
    max_time: Duration
) -> Result<F::Output, Duration>
{
    match trpl::select(f, trpl::sleep(max_time)).await {
        Either::Left(left) => Ok(left),
        Either::Right(_) => Err(max_time)
    }
}
