use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
/**
ThreadId(2): sent Message 1
ThreadId(2): sent Message 2
ThreadId(2): sent Message 3
ThreadId(2): sent Message 4
ThreadId(2): sent Message 5
ThreadId(2): sent Message 6
ThreadId(2): sent Message 7
ThreadId(2): sent Message 8
ThreadId(2): sent Message 9
ThreadId(2): done
Main: got Message 1
Main: got Message 2
Main: got Message 3
Main: got Message 4
Main: got Message 5
Main: got Message 6
Main: got Message 7
Main: got Message 8
Main: got Message 9
*/