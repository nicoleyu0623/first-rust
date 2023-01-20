

fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");


    // `Result` is the standard type to implement error handling
    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
    // idx: Ok(0) <--- Result::Ok hold the index of the element found
        // idx: Err <--- Result::Err hold the index of the element not found
}