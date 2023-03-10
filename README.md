# just my quick notes while learning Rust ... 

# Scalar Types

|                        | Types                                      | Literals                      |
|------------------------|--------------------------------------------|-------------------------------|
| Signed integers        | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123i64` |
| Unsigned integers      | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10u16`           |
| Floating point numbers | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2f32`    |
| Strings                | `&str`                                     | `"foo"`, `r#"\\"#`            |
| Unicode scalar values  | `char`                                     | `'a'`, `'α'`, `'∞'`           |
| Byte strings           | `&[u8]`                                    | `b"abc"`, `br#" " "#`         |
| Booleans               | `bool`                                     | `true`, `false`               |

The types have widths as follows:

* `iN`, `uN`, and `fN` are _N_ bits wide,
* `isize` and `usize` are the width of a pointer,
* `char` is 32 bit wide,
* `bool` is 8 bit wide.


# Common types

-  `Option` and `Result` types: used for optional values and error handling.

- `String`: the default string type used for owned data.

- `Vec`: a standard extensible vector.

- `HashMap`: a hash map type with a configurable hashing algorithm.

- `Box`: an owned pointer for heap-allocated data.

- `Rc`: a shared reference-counted pointer for heap-allocated data.
