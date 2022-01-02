/*
PRIMITIVE TYPES

!! the number indicates how many bits it takes in memory

Integers - i8, u8, i16, u16, i32, u32, i64, u64, i128, u128
    note: u8, u16, etc means unsigned - can only be positive
Floats (decimals) - f32, f64
Booleans (true/false) - bool
Characters - char
Tuples
Arrays

Rust is statically typed - it must know the types at compile time
HOWEVER, it can usually infer based on context.
*/

pub fn run() {

 // ~INTS & FLOATS~

    // default for ints is i32
    let x = 1;

    // default for floats is f64
    let y = 2.5;

    // explicit typing
    let z: i64 = 454545454545;

    // find max size using standard library
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);


 // ~BOOLEANS~

    let is_active: bool = true;

    // get bool from expression
    let is_greater = 10 > 5;

 // ~CHARACTERS~
    let a1 = 'a'; // !! use single quotes for characters!
    let face = '\u{1F600}'; // unicode for grin emoji

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}