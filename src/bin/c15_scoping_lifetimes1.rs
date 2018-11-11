// A lifetime is a construct that the compiler (the borrow checker) uses to ensure all borrows are valid.
// Specifically, a variables lifetime begins when it is created and ends when it is destroyed.
// Lifetimes and scopes are not the same.

// When we borrow a variable via `&`. The borrow has a lifetime that is determined by where it is declared. 
// As a result, the borrow is valid as long as it ends before the lender is destroyed. 
// However, the scope of the borrow is determined by where the reference is used. 

fn main() {
    let i = 3; // lifetime for i starts
    {
        let borrow1 = &i; // `borrow1` lifetime starts
        println!("borrow1: {}", borrow1);
    } // `borrow1` ends
    {
        let borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` ends

    // EXPLITCIT ANNOTATION
    let (four, nine) = (4, 9);
    print_refs(&four, &nine); // An input that borrowed must outlive the borrower
    failed_borrow();

    // FUNCITONS
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y); 
    print_one(&z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
} // i ends

// EXPLICIT ANNOTATION
// The borrow checker uses explicit lifetime annotations to detemine how long references should be valid.
// In cases where lifetimes are not elided*, rust requires explicit annotations to determine what the lifetime 
// of a reference should be
// * - elision implicitly annotates lifetimes

// syntax for explicitly annotating a lifetime uses an apostrophe: foo<'a> 
// Similar to closures, using lifetimes requires generics. Additionally, this lifetime syntax indicates that 
// the lifetime of foo may not exceed that of 'a. Explicit annotation has the form &'a T where 'a has
// already been introduced. In cases with multiple lifetimes, the syntax is similar `foo<'a, 'b>
// In this case, the lifetime of foo cannot exceed either of 'a or 'b

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) { // print_refs takes 2 references to i32 which have different lifetimes
    println!("x is {} and y is {}", x, y); // The 2 lifetimes must at least be as long as the funtion `print_refs`
}

// A function that takes no arguments but has a lifetime parameter
fn failed_borrow<'a>() {
    let _x = 12;
    // let y: &'a i32 = &_x; // This cannot be used as the lifetime of _x is shorter than y
    // A shorter lifetime cannot be coerced into a longer one.
}

// FUNCITONS
// Ignoring elision, function signatures with lifetimes have a few constraints
// * Any reference must have an annotated lifetime
// * Any reference being returned must have the same lifetime as the input or be static
// Returning a reference without an input is banned if it would result in returning references to invalid data.

fn print_one<'a>(x: &'a i32) { // One input reference with lifetime 'a which must live at least as long as
    println!("print_one: x is {}", x); // the function
}

fn add_one<'a>(x: &'a mut i32) { // mutable references are possible with lifetimes as well
    *x += 1;
}

// A function can have multiple elements - with either different or same lifetimes
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

// Returning references that have been passed in is acceptable. 
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// invalid_output is invalid: 'a must live longer than the function.
// &String::from("foo") would create a string followed by a reference. The data is dropped upon exiting the scope
// leaving a reference to invalid data to be returned.
