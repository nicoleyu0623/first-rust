fn main() {
    let mut x = 10;
    if x % 2 == 0 {
        x = x / 2;
        println!("{x}")
    } else {
        x = 3 * x + 1;
    }
}
// or


fn main() {
    let mut x = 10;
    x = if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    };
}

// if let

fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {              // sometimes working with `Option`
        println!("Program name: {value}"); // go to here: more concise than `match`
    } else {
        println!("Missing name?");
    }
}


