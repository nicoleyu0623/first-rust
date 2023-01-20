fn main() {
    let s1: &str = "Hello"; // &str: immutable reference in memory
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello "); // String: mutable
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}