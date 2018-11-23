#[allow(unused)]

use std::mem;

fn main() {
    let _logical: bool = true;

    let _a_float: f64 = 1.0; // regular annotation
    let _an_integer = 5i32; // suffix annotation

    let _default_float = 64.0; // f64
    let _default_integer = 32; // i32

    // while a mutable type's value can be changed, the type of the variable can't be
    let _integer_new: i32 = 32;
    // integer_new: i64 = 64;  // this will not compile

    // variables can be overwritten by shadowing
    let _integer_new: i64 = 64;

    println!("{}", 0.000001 == 0.000_001); // _ can be used in numeric literals to improve readability

    println!("1 - 2 = {}", 1i32 - 2); // a u32 here will not work

    // short-circuiting boolean logic
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    // Bitwise operations
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // TUPLES: can hold multiple types/values and hold any number of values; can be used by functions to return multiple objects
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("long tuple first value is {}", long_tuple.0);
    println!("long tuple second value is {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuples inside a tuple {:?}", tuple_of_tuples); // remember that long_tuples cannot be printed

    let pair = (1, false);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // (1i32) is not a tuple but an integer; (i32, ) is a 1 element tuple

    // destructing a tuple
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // ignored a matrix print


    // ARRAYS and SLICES: the signature of an array has 2 elements: type and length
    // arrays are allocated on the stack
    // slices are similar to arrays but their size is not known at compile time
    // a slice is a 2 word object: first is a pointer to the data, second is the length of the slice
    let xs: [i32; 5] = [0, 1, 2, 3, 4];
    let ys: [i32; 500] = [0; 500]; // all elements can be initialized to the same value

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs)); // arrays are stack allocated

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a selection of array as a slice");
    analyze_slice(&ys[1 .. 4]);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let can be used to bind the members of a tuple to variables

    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn analyze_slice(slice: &[i32]) {
    println!("the first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}