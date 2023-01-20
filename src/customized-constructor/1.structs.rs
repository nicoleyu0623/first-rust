/*
    Simple constructor
 */

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    println!("{} is {} years old", peter.name, peter.age);
}


/*
    Tuple structs
 */
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}

/*
    with new()
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
