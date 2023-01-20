fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    /*
     ownership already not exists after first function call
     */

    //say_hello(name);

}


//1. resolve via accepting a reference as a parameter

fn say_hello(name: &str) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(&name);
    say_hello(&name);
}

// 2. resolve via name.clone()
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name.clone());
}
