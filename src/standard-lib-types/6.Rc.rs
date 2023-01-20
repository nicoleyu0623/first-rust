use std::rc::Rc;

fn main() {
    let mut a = Rc::new(10);
    let mut b = a.clone();

    println!("a: {a}");
    println!("b: {b}");
}


// Rc is a reference-counted shared pointer.
// Use this when you need to refer to the same data from multiple places
