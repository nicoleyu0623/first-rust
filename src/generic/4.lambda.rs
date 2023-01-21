/*
  lambda expression have types which cannot be named.
  they implement special `Fn` , `FnMut`, `FnOnce`

  - `Fn`: not consume or mutate anything, can be called multiple times concurrently.
  - `FnMut`: may mutate captured variables. Call it multiple times but not concurrently.
  - `FnOnce`: may only called it once. Might consume captured values.
 */

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3;
    let mul_5 = |x| x * 5;

    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 20));
}


/*

Calling function on 10
add_3: 13
Calling function on 20
mul_5: 100

 */
