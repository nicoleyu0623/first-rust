use std::thread;

fn main() {
    let s = String::from("Hello");

    // borrow variable from outside scope

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}