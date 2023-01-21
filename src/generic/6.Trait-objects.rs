use std::fmt::Display;

fn main() {
    let xs: Vec<
        Box<dyn Display> // box of interfaced objects
    > = vec![
        Box::new(123),
        Box::new("Hello")
    ];

    for x in xs {
        println!("x: {x}");
    }

}

/*
   Trait object if you want to return different values
   implementing a trait

 */

fn numbers(n: i32) -> Box<dyn Iterator<Item=i32>> {
    if n > 0 {
        Box::new(0..n)
    } else {
        Box::new((n..0).rev())
    }
}

fn main() {
    println!("{:?}", numbers(-5).collect::<Vec<_>>());
    println!("{:?}", numbers(5).collect::<Vec<_>>());
}
/*
    [-1, -2, -3, -4, -5]
    [0, 1, 2, 3, 4]
 */