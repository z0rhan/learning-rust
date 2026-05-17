use std::thread;
use std::time::Duration;

use std::sync::mpsc;

use std::sync::{Mutex, Arc};

fn main() {
    using_threads();
    message_passing();
    shared_state();

    // Send and Sync trait from std::marker
    // Send trait indicates that the type that implements it can be send
    // between threads
    // Sync trait indicates it is safe for the type that implements it to be
    // referenced from multiple threads
    // We don't have to implement those traits manually
    // Any type with types that implements them will also automatically
    // implement them
    // Manually implementing them involves unsafe Rust code

    // My attempt at creating a deadlock
    let mutex1 = Arc::new(Mutex::new(5));
    let mutex2 = Arc::new(Mutex::new(5));

    let mut11 = Arc::clone(&mutex1);
    let mut12 = Arc::clone(&mutex2);
    let handle1 = thread::spawn(move || {
        let num1 = mut11.lock().unwrap();
        thread::sleep(Duration::from_millis(1));
        let num2 = mut12.lock().unwrap();

        let sum = *num1 + *num2;
        println!("Adding {num1} and {num2}: {sum}")
    });
    let mut21 = Arc::clone(&mutex1);
    let mut22 = Arc::clone(&mutex2);
    let handle2 = thread::spawn(move || {
        let num2 = mut22.lock().unwrap();
        thread::sleep(Duration::from_millis(1));
        let num1 = mut21.lock().unwrap();

        let sum = *num1 + *num2;
        println!("Adding {num1} and {num2}: {sum}")
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn shared_state() {
    // Rc<T> is not safe to share accross threads, we have Arc<T> for that
    // let m = rc::new(Mutex::new(6));
    let m = Arc::new(Mutex::new(6));
    {
        // .lock() blocks the current thread until it gets the lock
        // It returns MutexGuard, wrapped with LockResult
        let mut num = m.lock().unwrap();
        // MutexGuard implements Deref that allows to access the inner data
        // Also, implements Drop which releases the lock when out of scope
        *num = 0;
    }
    println!("m: {m:?}");

    let mut handles = vec![];
    for _ in 1..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            // Here, we can see that Mutex<T> provides interior mutability
            let mut num = m.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m: {m:?}");
    {
        let mut num = m.lock().unwrap();
        *num += 10;
    }
    println!("m: {m:?}");
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let str_list = vec![
            String::from("Hello "),
            String::from("from "),
            String::from("the "),
            String::from("other "),
            String::from("side"),
        ];
        for str in str_list {
            tx.send(str).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let str_list = vec![
            String::from("Hello "),
            String::from("again "),
            String::from("from "),
            String::from("the "),
            String::from("other "),
            String::from("side"),
        ];
        for str in str_list {
            tx1.send(str).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // .recv() blocks the thread until a value is received
    // If the transmitter closes, it produces an Error
    // .try_recv() will not block but return an Result immediately
    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {received}");
    }
}

fn using_threads() {
    // Without joining the handle, the execution of the spwaned is not
    // guaranteed and not deterministic also
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{i} Hi from spwaned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The placement of .join() also affects if the threads run at the same time
    handle.join().unwrap();

    for i in 1..5 {
        println!("{i} Hi from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let list = vec![1, 2, 3, 4, 5];
    // Here we should make the spwaned thread take ownership of list
    // else the thread cannot tell if the ref it holds is valid or not
    let handle = thread::spawn(move || {
        println!("Here is some vec: {:?}", list);
    });
    handle.join().unwrap();
}
