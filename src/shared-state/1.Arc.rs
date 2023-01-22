use std::thread;
use std::sync::Arc;

fn main() {

    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        // allow read-only access via it's clone() method
        let v = v.clone();
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}

/*
ThreadId(4): [10, 20, 30]
ThreadId(3): [10, 20, 30]
ThreadId(2): [10, 20, 30]
ThreadId(5): [10, 20, 30]
v: [10, 20, 30]
 */