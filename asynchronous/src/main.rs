use std::{env, thread, time::Duration};

use trpl::{Either, Html, StreamExt};

use asynchronous;

// So basically, async functions/blocks return something that implments the
// Future trait which is lazy by default and needs a runtime to execute
// Rust provides some kind of state machine for the different states of async
// code which are then utilized by runtimes to schedule the many async code

fn main() {
    let args: Vec<String> = env::args().collect();

    trpl::block_on(async {
        let url1_fu = page_tile(&args[1]);
        let url2_fu = page_tile(&args[2]);

        let (url, maybe_title) =
            match trpl::select(url1_fu, url2_fu).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("It had page title: {title}"),
            None => println!("It had not title")
        }
    });

    trpl::block_on( async {
        concurrency_with_async().await;
        msg_passing().await;
        yeilding_control_to_runtime().await;

        match asynchronous::timeout(takes_5_sec().await, Duration::from_millis(2000)).await {
            Ok(_) => println!("The task finished within given time limit"),
            Err(_) => println!("The task did not finish even after given time limit")
        };

        stream().await;
    });
}

async fn takes_5_sec() -> impl Future {
    trpl::sleep(Duration::from_millis(5000))
}

fn slow(name:& str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

async fn yeilding_control_to_runtime() {
    // So anything between await is synchronous code hence if async has no
    // await, it will block other futures
    // So, we use yield_now to hand control to runtime so that it can also
    // run other futures if possible between these sync code that needs some
    // time
    let a = async {
        println!("a started");
        slow("a", 10);
        trpl::yield_now().await;
        slow("a", 20);
        trpl::yield_now().await;
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 15);
        trpl::yield_now().await;
        println!("a finished");
    };

    let b = async {
        println!("b started");
        slow("b", 10);
        trpl::yield_now().await;
        slow("b", 20);
        trpl::yield_now().await;
        slow("b", 30);
        trpl::yield_now().await;
        slow("b", 15);
        trpl::yield_now().await;
        println!("b finished");
    };

    trpl::select(a, b).await;
}

async fn msg_passing() {
    let (tx, mut rx) = trpl::channel();
    let tx1 = tx.clone();

    let tx_fut = async move {
        let vals = vec![
            String::from("First"),
            String::from("Second"),
            String::from("Third"),
            String::from("Forth"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let rx_fut = async {
        // Here in contrast to msg passing with threads, it does not block current
        // thread, it hands control to runtime until msg is received or the
        // transmitter is closed
        while let Some(val) = rx.recv().await {
            println!("Received: {val}")
        }
    };

    let tx1_fut = async move {
        let vals = vec![
            String::from("First"),
            String::from("Second"),
            String::from("Third"),
            String::from("Forth"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    // For multiple producers we can use the join! macro to join all the futures
    let _ = trpl::join!(tx_fut, rx_fut, tx1_fut);
}

async fn stream() {
    let values = vec![1, 2, 3, 4, 5, 6];
    let iter = values.iter().map(|n| n * 2);

    let mut stream = trpl::stream_from_iter(iter);

    while let Some(val) = stream.next().await {
        println!("The value was {val}")
    }
}

async fn concurrency_with_async() {
    let handle = trpl::spawn_task(async {
        for i in 1..10 {
            println!("Hello {i} from first task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    let fut1 = async {
        for i in 1..5 {
            println!("Hello {i} from second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("Hello {i} from third task");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    // This returns a new future whoose output is a tuple of output of both
    // of the future given to it
    trpl::join(fut1, fut2).await;

    // Like with threads, the task will stop executing once the main 
    // program ends, so we need to use await that does something like join()
    handle.await.unwrap();
}

async fn page_tile(url: &str) -> (&str, Option<String>) {
    let res_text = trpl::get(url).await.text().await;

    let title = Html::parse(&res_text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}

// This roughly the same as
// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let res_text = trpl::get(url).await.text().await;
// 
//         Html::parse(&res_text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }
