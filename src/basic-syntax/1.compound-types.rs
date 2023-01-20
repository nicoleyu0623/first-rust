// array assignment and access
fn array() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a); // adding debug output
}
// a: [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]

// tuple assignment and access
fn tuple() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}

// vector
fn vector() {
    // vector
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    // vector with generic type
    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}

// compile time constant
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);
// mutable global variables
static BANNER: &str = "Welcome to RustOS 3.14";

// shadow variables
// this works ....
fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

// iterating
fn main() {
    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}
