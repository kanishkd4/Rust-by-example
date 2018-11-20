// STATIC
// A 'static lifetime is the longest possible lifetime and lasts for the lifetime of the program. 
// A 'static lifetime may also be coerced to a shorter lifetime. There are ways to make a variable with 'static lifetime,
// and both are stored in the read only memory of of the binary
// 1. Make a constant with the 'static declaration
// 2. Make a string literal which has the type `&' static str`

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 { 
    &NUM // returns a reference to NUM where its 'static lifetime is coerced to that of the input argument
}

fn main() {
    {
        // Make a string literal and print it
        let static_string = "I'm in read only memory";
        println!("static_string: {}", static_string);
        // When `static_string` goes out of scope, the reference can no longer be used but the data remains in the binary
    }
    {
        // Make an integer to use for `coerce_static`
        let lifetime_num = 9;
        // Coerce 'NUM' to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible", NUM);

    // ELISION
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotaed_pass(&x));
}

// ELISION
// Some lifetime parameters are so common the borrow checker will implicitly add them to save typing and to improve readability
// The process of implicit addition is called elision

// `elided_input` and `annotated_input` essentially have identical signatures because the lifetime of 
// `elided_input` is elided by the compiler
fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}

// Similarly, elided_pass and annotated_pass have identical signatures because the lifetime is added implicitly to elided_pass
fn elided_pass(x: &i32) -> &i32 { x }
fn annotaed_pass<'a>(x: &'a i32) -> &'a i32 { x }