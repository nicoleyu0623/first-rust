fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    println!("x: {ref_x}"); // 10
    *ref_x = 20; // dereference ref_x when assigning to it
    println!("x: {x}"); // 20
}