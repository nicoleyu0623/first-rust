use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}

// Hello foo

/*
- 1. as a parameter: anon generic parameter with a trait bound
- 2. as a return type: concrete type that implements the trait, without naming the type

// when don't want to expose the concrete type in a public API

*/


